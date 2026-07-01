import { build } from 'esbuild';
import { writeFileSync, readFileSync } from 'node:fs';

async function main() {
  const result = await build({
    entryPoints: ['src/index.ts'],
    bundle: true,
    platform: 'node',
    target: 'node20',
    format: 'esm',
    outfile: 'dist/companion.js',
    minify: false,
    sourcemap: true,
    external: ['yaml'],
    banner: {
      js: '#!/usr/bin/env node\n',
    },
    define: {
      'import.meta.dirname': 'import.meta.dirname',
    },
  });

  console.log('Build complete:', result);
}

main().catch((err) => {
  console.error('Build failed:', err);
  process.exit(1);
});
