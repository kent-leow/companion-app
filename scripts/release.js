#!/usr/bin/env node
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const bump = process.argv[2];
if (!['major', 'minor', 'hotfix'].includes(bump)) {
  console.error('Usage: npm run release -- <major|minor|hotfix>');
  process.exit(1);
}

const pkgPath = path.join(__dirname, '..', 'application', 'package.json');
const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
const [major, minor, patch] = pkg.version.split('.').map(Number);

const newVersion =
  bump === 'major' ? `${major + 1}.0.0` :
  bump === 'minor' ? `${major}.${minor + 1}.0` :
  `${major}.${minor}.${patch + 1}`;

pkg.version = newVersion;
fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');

const lockPath = path.join(__dirname, '..', 'application', 'package-lock.json');
if (fs.existsSync(lockPath)) {
  const lock = JSON.parse(fs.readFileSync(lockPath, 'utf8'));
  lock.version = newVersion;
  fs.writeFileSync(lockPath, JSON.stringify(lock, null, 2) + '\n');
}

execSync(`git add -A && git commit -m "release: v${newVersion}"`, { stdio: 'inherit' });
execSync(`git tag v${newVersion}`, { stdio: 'inherit' });
execSync(`git push origin main --follow-tags`, { stdio: 'inherit' });

console.log(`\n✅ Released v${newVersion} — pipeline will build and publish.`);
