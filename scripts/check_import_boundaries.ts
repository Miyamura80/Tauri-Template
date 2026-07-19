#!/usr/bin/env bun
/**
 * Crate-boundary check.
 *
 * The core crate `crates/engine` is platform-agnostic and MUST NOT depend on
 * any transport crate. Transports (the `appctl` CLI in `crates/cli`, the Tauri
 * host in `src-tauri`) depend on `engine`, never the reverse. This asserts that
 * dependency direction by parsing the Cargo.toml manifests. Deterministic, no
 * network, no cargo invocation.
 */

import { readFileSync } from "node:fs";
import { join } from "node:path";

const REPO = join(import.meta.dir, "..");

// The core crate and the manifest that must stay free of transport deps.
const CORE_CRATE = "engine";
const CORE_MANIFEST = join(REPO, "crates", "engine", "Cargo.toml");

// Transport crates the core is forbidden from depending on. `name` is the
// crate/package name as it would appear in a `[dependencies]` key.
const FORBIDDEN_DEPS: Array<{ name: string; label: string }> = [
	{ name: "appctl", label: "crates/cli (appctl CLI transport)" },
	{ name: "tauri-app", label: "src-tauri (Tauri host transport)" },
	{ name: "tauri", label: "the Tauri framework (GUI transport)" },
	{ name: "tauri-build", label: "the Tauri build framework (GUI transport)" },
];

/**
 * Collect the dependency keys from a Cargo.toml. Naive but sufficient: scan
 * every `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`, and
 * their `[target.*]` variants, then read `key = ...` and `key.path = ...`
 * lines plus any `dep = { ... }` table headers under those sections.
 */
function collectDependencyNames(manifest: string): Set<string> {
	const deps = new Set<string>();
	const lines = manifest.split("\n");
	let inDepsSection = false;

	const isDepSectionHeader = (header: string): boolean =>
		/^(dependencies|dev-dependencies|build-dependencies)$/.test(header) ||
		/(^|\.)(dependencies|dev-dependencies|build-dependencies)$/.test(header);

	for (const raw of lines) {
		const line = raw.trim();
		if (line.startsWith("#") || line === "") continue;

		const tableMatch = line.match(/^\[([^\]]+)\]$/);
		if (tableMatch) {
			const header = tableMatch[1].trim();
			// A `[dependencies.foo]` table both enters a deps section AND names `foo`.
			const nested = header.match(
				/^(?:target\.[^.]+\.)?(?:dependencies|dev-dependencies|build-dependencies)\.(.+)$/,
			);
			if (nested) {
				deps.add(nested[1].trim());
				inDepsSection = false;
				continue;
			}
			inDepsSection = isDepSectionHeader(header);
			continue;
		}

		if (!inDepsSection) continue;

		const keyMatch = line.match(
			/^([A-Za-z0-9_-]+)\s*(?:\.[A-Za-z0-9_-]+)?\s*=/,
		);
		if (keyMatch) deps.add(keyMatch[1].trim());
	}
	return deps;
}

function main(): number {
	let manifest: string;
	try {
		manifest = readFileSync(CORE_MANIFEST, "utf-8");
	} catch (e) {
		console.error(`Could not read ${CORE_MANIFEST}: ${e}`);
		return 1;
	}

	const deps = collectDependencyNames(manifest);
	const violations = FORBIDDEN_DEPS.filter((f) => deps.has(f.name));

	if (violations.length > 0) {
		console.error(
			`Import boundary violation: core crate \`${CORE_CRATE}\` (crates/engine/Cargo.toml) ` +
				`must not depend on transport crates.`,
		);
		for (const v of violations) {
			console.error(`  forbidden dependency: \`${v.name}\` -> ${v.label}`);
		}
		console.error(
			"Transports depend on the engine, never the reverse. Move shared logic into the engine.",
		);
		return 1;
	}

	console.log(
		`Import boundary check passed: \`${CORE_CRATE}\` has no transport-crate dependency.`,
	);
	return 0;
}

process.exit(main());
