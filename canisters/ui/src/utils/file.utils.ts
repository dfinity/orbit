export const readFileAsArrayBuffer = (file: File): Promise<ArrayBuffer> => {
  const reader = new FileReader();

  reader.readAsArrayBuffer(file);

  return new Promise((resolve, reject) => {
    reader.onload = () => {
      resolve(reader.result as ArrayBuffer);
    };

    reader.onerror = reject;
  });
};
