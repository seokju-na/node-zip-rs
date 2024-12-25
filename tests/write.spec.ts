import { Buffer } from 'node:buffer';
import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it, test } from 'vitest';
import { ZipArchive, ZipWriter, openZipArchiveAsync, writeZipAsync } from '../index';
import { ROOT_DIR } from './env';

test('write zip file', async () => {
  const srcDir = path.join(ROOT_DIR, 'tests', 'fixtures', 'simple');
  const dst = path.join(ROOT_DIR, 'tmp', 'simple.zip');
  await fs.mkdir(path.dirname(dst), { recursive: true });
  await writeZipAsync(srcDir, dst);
  await expect(fs.access(dst)).resolves.toBeUndefined();
});

describe('Writer', () => {
  it('finish zip writing and create file', async () => {
    const realpath = path.join(ROOT_DIR, 'tests', 'fixtures', 'simple', 'index.js');
    const zip = new ZipWriter();
    zip.writeFile('index.js', realpath);
    zip.finish(path.join(ROOT_DIR, 'tmp', 'simple_written.zip'));
    const archive = await openZipArchiveAsync(path.join(ROOT_DIR, 'tmp', 'simple_written.zip'));
    await expect(archive.readFileAsync('index.js')).resolves.toEqual(
      Buffer.from("console.log('Hello World');", 'utf8')
    );
  });

  it('finish zip writing and returns buffer', async () => {
    const realpath = path.join(ROOT_DIR, 'tests', 'fixtures', 'simple', 'index.js');
    const zip = new ZipWriter();
    zip.writeFile('index.js', realpath);
    const buf = zip.finishToBuffer();
    const archive = ZipArchive.fromBuffer(buf);
    await expect(archive.readFileAsync('index.js')).resolves.toEqual(
      Buffer.from("console.log('Hello World');", 'utf8')
    );
  });
});
