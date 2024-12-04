import { Buffer } from 'node:buffer';
import path from 'node:path';
import { expect, test } from 'vitest';
import { openArchiveAsync } from '../index.js';
import { ROOT_DIR } from './env';

test('fileNames', async () => {
  const filepath = path.join(ROOT_DIR, 'tests', 'fixtures', 'nextjs.zip');
  const archive = await openArchiveAsync(filepath);
  expect(archive.fileNames()).toContain('_next/static/css/6fce7568a752b0cf.css');
});

test('extractAsync', async () => {
  const filepath = path.join(ROOT_DIR, 'tests', 'fixtures', 'nextjs.zip');
  const archive = await openArchiveAsync(filepath);
  await archive.extractAsync(path.join(ROOT_DIR, 'tmp', 'nextjs-extract'));
});

test('readFileAsync', async () => {
  const filepath = path.join(ROOT_DIR, 'tests', 'fixtures', 'nextjs.zip');
  const archive = await openArchiveAsync(filepath);
  const file = await archive.readFileAsync('_next/static/chunks/main-e10b097cefff9b8a.js');
  expect(file).instanceOf(Buffer);
});

test('readFileAsync - error if directory', async () => {
  const filepath = path.join(ROOT_DIR, 'tests', 'fixtures', 'nextjs.zip');
  const archive = await openArchiveAsync(filepath);
  await expect(archive.readFileAsync('_next/static/chunks/')).rejects.toThrowError(/file not found/);
});
