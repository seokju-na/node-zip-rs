import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { bench, describe } from 'vitest';
import yauzl from 'yauzl-promise';
import zipRs from '../';

const dirname = path.basename(fileURLToPath(import.meta.url));
const filepath = path.join(dirname, '..', 'tests', 'fixtures', 'nextjs.zip');

function streamToBuffer(stream: NodeJS.ReadableStream) {
  return new Promise<Buffer>((resolve, reject) => {
    const chunks: any[] = [];

    stream.on('data', chunk => chunks.push(chunk));
    stream.on('end', () => resolve(Buffer.concat(chunks)));
    stream.on('error', err => reject(err));
  });
}

describe('readFileAsync', () => {
  bench('yauzl', async () => {
    const zip = await yauzl.open(filepath);
    try {
      for await (const entry of zip) {
        if (entry.filename.endsWith('/')) {
          continue;
        }
        const read = await entry.openReadStream();
        await streamToBuffer(read);
      }
    } finally {
      await zip.close();
    }
  });

  bench('zip-rs', async () => {
    const zip = await zipRs.openArchiveAsync(filepath);
    for (const file of zip.fileNames()) {
      await zip.readFileAsync(file);
    }
  });
});
