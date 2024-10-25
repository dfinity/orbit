import { Certificate, LookupStatus } from '@dfinity/agent';
import { decode, encode, getServiceMethods } from '@dfinity/didc';
import { Principal } from '@dfinity/principal';
import { icAgent } from '~/core/ic-agent.core';

export { decode, encode, getServiceMethods };

export const fetchCanisterIdlFromMetadata = async (
  canisterId: Principal,
  agent = icAgent.get(),
): Promise<string | undefined> => {
  const encoder = new TextEncoder();
  const versionPath: ArrayBuffer[] = [
    encoder.encode('canister'),
    canisterId.toUint8Array(),
    encoder.encode('metadata'),
    encoder.encode('candid:service'),
  ];

  const state = await agent.readState(canisterId, {
    paths: [versionPath],
  });

  const certificate = await Certificate.create({
    canisterId,
    certificate: state.certificate,
    rootKey: agent.rootKey,
  });

  const serviceIdl = certificate.lookup(versionPath);

  if (serviceIdl.status !== LookupStatus.Found) {
    return undefined;
  }

  if (!(serviceIdl.value instanceof ArrayBuffer)) {
    throw new Error('candid:service metadata is not an ArrayBuffer');
  }

  const decoder = new TextDecoder();
  const decodedServiceIdl = decoder.decode(serviceIdl.value);

  return decodedServiceIdl.trim()?.length ? decodedServiceIdl.trim() : undefined;
};
