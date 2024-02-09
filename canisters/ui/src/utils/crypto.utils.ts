export const arrayBufferToHex = (buffer: ArrayBuffer): string => {
  return Array.from(new Uint8Array(buffer))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');
};

export const arrayBufferToHashHex = async (
  buffer: ArrayBuffer,
  algorithm: AlgorithmIdentifier = 'SHA-256',
): Promise<string> => {
  const hash = await crypto.subtle.digest(algorithm, buffer);

  return arrayBufferToHex(hash);
};

export const hexStringToArrayBuffer = (hexString: string): ArrayBuffer => {
  // Ensure the hex string has an even length
  if (hexString.length % 2 !== 0) {
    throw new Error('Hex string must have an even length');
  }

  // Convert hex string to byte array
  const byteArray = new Uint8Array(hexString.length / 2);
  for (let i = 0; i < hexString.length; i += 2) {
    byteArray[i / 2] = parseInt(hexString.substring(i, i + 2), 16);
  }

  // Create an ArrayBuffer from the byte array
  const arrayBuffer = byteArray.buffer;

  return arrayBuffer;
};
