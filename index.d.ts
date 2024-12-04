/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare function openArchive(path: string): Archive
export declare function openArchiveAsync(path: string, signal?: AbortSignal | undefined | null): Promise<Archive>
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
export declare function write(srcDir: string, dst: string, options?: WriteFileOptions | undefined | null): void
export declare function writeAsync(srcDir: string, dst: string, options?: WriteFileOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<void>
export declare class Archive {
  isEmpty(): boolean
  readFile(name: string): Buffer
  readFileAsync(name: string, signal?: AbortSignal | undefined | null): Promise<Buffer>
  extract(outdir: string): void
  extractAsync(outdir: string, signal?: AbortSignal | undefined | null): Promise<void>
  fileNames(): Array<string>
}