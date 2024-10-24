import { Certificate, LookupStatus } from '@dfinity/agent';
import loadDidcLib, { decode, encode, getServiceMethods } from '@dfinity/didc';
import { Principal } from '@dfinity/principal';
import { icAgent } from '~/core/ic-agent.core';

// Load the didc library wasm file, this check is required because tests are run in nodejs where the wasm
// loading is synchronous and does not require the promise to be resolved as in the web.
if (typeof loadDidcLib === 'function') {
  await loadDidcLib();
}

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
