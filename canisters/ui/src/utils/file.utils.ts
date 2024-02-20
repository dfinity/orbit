import { CsvTable } from '~/types/app.types';

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

const saveCsvValue = (value: string): string => {
  if (value.includes('"')) {
    value = value.replace(/"/g, '""');
  }

  if (value.includes(',') || value.includes('"')) {
    value = `"${value}"`;
  }

  return value;
};

const csvToBlob = (csv: CsvTable): string => {
  const headerValues: string[] = [];
  for (const headerKey in csv.headers) {
    const headerValue = saveCsvValue(csv.headers[headerKey]);

    headerValues.push(headerValue);
  }

  const header = headerValues.join(',');
  let rows: string = '';
  for (const rowKey in csv.rows) {
    const row = csv.rows[rowKey];
    const rowValues = [];
    for (const headerKey in csv.headers) {
      const rowValue = saveCsvValue(row?.[headerKey] ?? '');

      rowValues.push(rowValue);
    }

    rows += rowValues.join(',') + '\r\n';
  }

  return `${header}\r\n${rows}`;
};

export const downloadCsv = async ({
  content,
  filename,
}: {
  content: CsvTable;
  filename?: string;
}): Promise<void> => {
  const blob = new Blob([csvToBlob(content)], { type: 'text/csv' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  const fileDownloadName = filename || 'download';

  a.href = url;
  a.download = fileDownloadName.endsWith('.csv') ? fileDownloadName : `${filename}.csv`;
  a.click();

  URL.revokeObjectURL(url);
};
