import { bufFromBufLike, Certificate, HttpAgent, lookupResultToBuffer } from '@dfinity/agent';
import { decode, encode } from '@dfinity/didc';
import { Principal } from '@dfinity/principal';
import { uint8ArrayToHexString } from '@dfinity/utils';
import { hexStringToUint8Array } from './helper.utils';

export class CustomIdlAgent {
  constructor(
    private config: {
      agent: HttpAgent;
      idl: string;
      canisterId: string | Principal;
    },
  ) {}

  async update(methodName: string, arg: ArrayBuffer | string) {
    let argBuffer: ArrayBuffer;
    if (typeof arg === 'string') {
      arg = encode({
        idl: this.config.idl,
        input: arg,
        withType: {
          kind: 'methodParams',
          name: methodName,
        },
        targetFormat: 'blob',
      });

      argBuffer = new TextEncoder().encode(arg).buffer;
    } else {
      argBuffer = arg;
    }

    const result = await this.config.agent.call(this.config.canisterId, {
      methodName,
      arg: argBuffer,
      callSync: true,
    });

    if (result.response.ok) {
      if (result.response.body && 'certificate' in result.response.body) {
        const certificate = await Certificate.create({
          certificate: bufFromBufLike(result.response.body.certificate),
          rootKey: this.config.agent.rootKey!,
          canisterId: Principal.from(this.config.canisterId),
        });
        const path = [new TextEncoder().encode('request_status'), result.requestId];
        const status = new TextDecoder().decode(
          lookupResultToBuffer(certificate.lookup([...path, 'status'])),
        );

        switch (status) {
          case 'replied': {
            const reply = lookupResultToBuffer(certificate.lookup([...path, 'reply']))!;
            return decode({
              idl: this.config.idl,
              input: uint8ArrayToHexString(new Uint8Array(reply)),
              serviceMethod: methodName,
            });
          }
          case 'rejected': {
            const rejectCode = new Uint8Array(
              lookupResultToBuffer(certificate.lookup([...path, 'reject_code']))!,
            )[0];
            const rejectMessage = new TextDecoder().decode(
              lookupResultToBuffer(certificate.lookup([...path, 'reject_message']))!,
            );
            const error_code_buf = lookupResultToBuffer(
              certificate.lookup([...path, 'error_code']),
            );
            const error_code = error_code_buf
              ? new TextDecoder().decode(error_code_buf)
              : undefined;

            throw new Error(
              `Request ${result.requestDetails?.method_name} with ${result.requestId} rejected with code ${rejectCode}: ${rejectMessage} ${error_code}`,
            );
          }
          default:
            throw new Error(`Unknown request status: ${status}`);
        }
      }
    }
  }

  async query(methodName: string, arg: ArrayBuffer | string) {
    let argBuffer: ArrayBuffer;
    if (typeof arg === 'string') {
      arg = encode({
        idl: this.config.idl,
        input: arg,
        withType: {
          kind: 'methodParams',
          name: methodName,
        },
        targetFormat: 'hex',
      });

      argBuffer = hexStringToUint8Array(arg);
    } else {
      argBuffer = arg;
    }

    const response = await this.config.agent.query(this.config.canisterId, {
      methodName,
      arg: argBuffer,
    });

    if (response.status === 'replied') {
      const decoded = decode({
        idl: this.config.idl,
        input: uint8ArrayToHexString(new Uint8Array(response.reply.arg)),
        serviceMethod: methodName,
      });

      return decoded;
    } else if (response.status === 'rejected') {
      throw new Error(response.reject_message);
    } else {
      throw new Error('Unknown query response status');
    }
  }
}
