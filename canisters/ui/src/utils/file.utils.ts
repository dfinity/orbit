import { CsvRow, CsvTable } from '~/types/app.types';

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

export const readFileAsCsvTable = async (file: File): Promise<CsvTable> => {
  const reader = new FileReader();

  reader.readAsText(file);

  return new Promise((resolve, reject) => {
    reader.onload = () => {
      const csv = reader.result as string;
      const [header, ...rows] = csv.replace(/\r\n/g, '\n').split('\n');
      const headers = header.split(',');
      const table: CsvTable = {
        headers: {},
        rows: [],
      };

      for (const header of headers) {
        const headerKey = header.toLowerCase().replace(/ /g, '_');
        table.headers[headerKey] = header;
      }

      for (const row of rows) {
        const rowValues = row.split(',');
        const rowObject: CsvRow = {};

        for (let i = 0; i < headers.length; i++) {
          rowObject[headers[i]] = rowValues[i];
        }

        table.rows.push(rowObject);
      }

      // cleanup last empty row
      if (
        table.rows.length &&
        Object.values(table.rows[table.rows.length - 1]).every(value => !value)
      ) {
        table.rows.pop();
      }

      resolve(table);
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
