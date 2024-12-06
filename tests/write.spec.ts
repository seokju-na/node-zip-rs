import fs from 'node:fs/promises';
import path from 'node:path';
import { expect, test } from 'vitest';
import { writeAsync } from '../index';
import { ROOT_DIR } from './env';

test('write zip file', async () => {
  const srcDir = path.join(ROOT_DIR, 'tests', 'fixtures', 'simple');
  const dst = path.join(ROOT_DIR, 'tmp', 'simple.zip');
  await fs.mkdir(path.dirname(dst), { recursive: true });
  await writeAsync(srcDir, dst);
  await expect(fs.access(dst)).resolves.toBeUndefined();
});
