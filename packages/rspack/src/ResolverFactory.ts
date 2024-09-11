import * as binding from "@rspack/binding";
import { Resolver } from "./Resolver";
import { type Resolve, getRawResolve } from "./config";
import { ThreadsafeReadableNodeFS } from "./FileSystem";

type ResolveOptionsWithDependencyType = Resolve & {
	dependencyCategory?: string;
	resolveToContext?: boolean;
};

export class ResolverFactory {
	#binding: binding.JsResolverFactory;

	static __to_binding(
		resolver_factory: ResolverFactory
	): binding.JsResolverFactory {
		return resolver_factory.#binding;
	}

	constructor(fs: ThreadsafeReadableNodeFS) {
		this.#binding = new binding.JsResolverFactory(fs);
	}

	get(
		type: string,
		resolveOptions?: ResolveOptionsWithDependencyType
	): Resolver {
		const { dependencyCategory, resolveToContext, ...resolve } =
			resolveOptions || {};

		const binding = this.#binding.get(type, {
			...getRawResolve(resolve),
			dependencyCategory,
			resolveToContext
		});
		return new Resolver(binding);
	}
}
