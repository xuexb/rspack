import path from 'node:path';
import util from "node:util";
import type { FileMetadata, ThreadsafeNodeFS, ThreadsafeNodeInputFS } from "@rspack/binding";

import { type InputFileSystem, type OutputFileSystem, mkdirp, rmrf } from "./util/fs";
import { memoizeFn } from "./util/memoize";

const NOOP_FILESYSTEM: ThreadsafeNodeFS = {
	writeFile() {},
	removeFile() {},
	mkdir() {},
	mkdirp() {},
	removeDirAll() {}
};
class ThreadsafeReadableNodeFS implements ThreadsafeNodeInputFS{
	//readToString!: (path: string) => Promise<string> | string;
	readToBuffer!: (path: string) => Promise<Buffer> | Buffer;
	metadata!: (path: string) => Promise<FileMetadata> | FileMetadata;
	symlinkMetadata!: (path:string) => Promise<FileMetadata> | FileMetadata;
	canonicalize!: (path:string) => Promise<string> |string;
	constructor(fs?: InputFileSystem){
		if (!fs) {
			// This happens when located in a child compiler.
			Object.assign(this, NOOP_FILESYSTEM);
			return;
		}
		this.readToBuffer =(p:string) => {
		   const buffer = fs.readFileSync!(p);
		   return buffer
		};
		
		this.canonicalize = (p:string)=> {
			const linkedPath = fs!.readlinkSync!(p,{});
			const absolutePath= path.resolve(path.dirname(p), linkedPath);
			return absolutePath;
			
		};
		this.metadata =(p:string) => {
			const stat = fs.statSync!(p);
			const res= {
				isFile: stat.isFile(),
				isDir: stat.isDirectory(),
				isSymlink: stat.isSymbolicLink()
			};
			return res;
		};
		this.symlinkMetadata = (p:string) => {
			const stat = fs.lstatSync!(p);
			const res = {
				isFile: stat.isFile(),
				isDir: stat.isDirectory(),
				isSymlink: stat.isSymbolicLink()
			};
			return res;
		};
	}
	static __to_binding(fs?: InputFileSystem) {
		return new this(fs);
	}
}
class ThreadsafeWritableNodeFS implements ThreadsafeNodeFS {
	writeFile!: (name: string, content: Buffer) => Promise<void> | void;
	removeFile!: (name: string) => Promise<void> | void;
	mkdir!: (name: string) => Promise<void> | void;
	mkdirp!: (name: string) => Promise<string | void> | string | void;
	removeDirAll!: (name: string) => Promise<string | void> | string | void;

	constructor(fs?: OutputFileSystem) {
		if (!fs) {
			// This happens when located in a child compiler.
			Object.assign(this, NOOP_FILESYSTEM);
			return;
		}
		this.writeFile = memoizeFn(() => {
			
			return util.promisify(fs.writeFile.bind(fs))
		});
		this.removeFile = memoizeFn(() => util.promisify(fs.unlink.bind(fs)));
		this.mkdir = memoizeFn(() => util.promisify(fs.mkdir.bind(fs)));
		this.mkdirp = memoizeFn(() => util.promisify(mkdirp.bind(null, fs)));
		this.removeDirAll = memoizeFn(() => util.promisify(rmrf.bind(null, fs)));
	}

	static __to_binding(fs?: OutputFileSystem) {
		return new this(fs);
	}
}

export { ThreadsafeWritableNodeFS, ThreadsafeReadableNodeFS };
