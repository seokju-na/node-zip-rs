# node-zip-rs

```shell
$ yarn add node-zip-rs
```

## API

```ts
export declare function openZipArchive(path: string): ZipArchive
export declare function openZipArchiveAsync(path: string, signal?: AbortSignal | undefined | null): Promise<ZipArchive>
export const enum CompressionMethod {
  Stored = 0,
  Deflated = 1,
  Deflate64 = 2,
  Bzip2 = 3,
  Aes = 4,
  Zstd = 5,
  Lzma = 6,
  Xz = 7
}
export interface WriteFileOptions {
  compressionMethod?: CompressionMethod
  compressionLevel?: number
  permissions?: number
  largeFile?: boolean
}
export declare function writeZip(srcDir: string, dst: string, options?: WriteFileOptions | undefined | null): void
export declare function writeZipAsync(srcDir: string, dst: string, options?: WriteFileOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<void>
export declare class ZipArchive {
  static fromBuffer(buffer: Buffer): ZipArchive
  isEmpty(): boolean
  readFile(name: string): Buffer
  readFileAsync(name: string, signal?: AbortSignal | undefined | null): Promise<Buffer>
  extract(outdir: string): void
  extractAsync(outdir: string, signal?: AbortSignal | undefined | null): Promise<void>
  fileNames(): Array<string>
}
export declare class ZipWriter {
  constructor()
  writeFile(filepath: string, realpath?: string | undefined | null, options?: WriteFileOptions | undefined | null): void
  finish(dst: string): void
  finishToBuffer(): Buffer
}
```

## Benchmark

```
 RUN  v2.1.8 /Users/seokju.me/workspace/node-rs-zip

 ✓ benchmarks/archive.bench.ts (2) 1224ms
   ✓ readFileAsync (2) 1222ms
     name              hz     min      max     mean      p75      p99     p995     p999     rme  samples
   · yauzl        89.5126  9.9123  13.7344  11.1716  11.5436  13.7344  13.7344  13.7344  ±2.52%       45
   · node-zip-rs   170.79  5.4248  13.6432   5.8551   5.7539  13.6432  13.6432  13.6432  ±4.23%       86   fastest

 BENCH  Summary

  node-zip-rs - benchmarks/archive.bench.ts > readFileAsync
    1.91x faster than yauzl
```

Tested with MacBook Pro 16 (M1 Max, 2021).

## License

MIT License
