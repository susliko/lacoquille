import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const distDir = path.join(__dirname, '..', 'dist');

function findHtmlFiles(dir) {
  const results = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      results.push(...findHtmlFiles(fullPath));
    } else if (entry.name.endsWith('.html')) {
      results.push(fullPath);
    }
  }
  return results;
}

function extractLinks(html) {
  const regex = /href="([^"]+)"/g;
  const links = [];
  let match;
  while ((match = regex.exec(html)) !== null) {
    links.push(match[1]);
  }
  return links;
}

function isInternalLink(href) {
  if (!href.startsWith('/')) return false;
  if (href.startsWith('/api/')) return false;
  if (/^[a-z]+:\/\//.test(href)) return false;
  if (href.startsWith('mailto:')) return false;
  if (/\.[a-zA-Z0-9]+$/.test(href) && !href.endsWith('.html')) return false;
  return true;
}

function resolveLink(distDir, pathname) {
  let normalized = pathname.startsWith('/') ? pathname.slice(1) : pathname;
  if (normalized.endsWith('/')) {
    normalized = normalized.slice(0, -1);
  }

  const tryPaths = [
    path.join(distDir, normalized, 'index.html'),
    path.join(distDir, normalized + '.html'),
    path.join(distDir, normalized, 'index'),
  ];

  for (const p of tryPaths) {
    if (fs.existsSync(p) && fs.statSync(p).isFile()) {
      return { found: true, path: p };
    }
  }

  return { found: false, path: tryPaths[0] };
}

const htmlFiles = findHtmlFiles(distDir);
const brokenLinks = [];

for (const htmlFile of htmlFiles) {
  const content = fs.readFileSync(htmlFile, 'utf-8');
  const links = extractLinks(content);
  const relativeHtmlFile = path.relative(process.cwd(), htmlFile);

  for (const href of links) {
    if (!isInternalLink(href)) continue;

    try {
      const url = new URL(href, 'http://localhost');
      const pathname = url.pathname;
      const result = resolveLink(distDir, pathname);

      if (!result.found) {
        brokenLinks.push({ href, source: relativeHtmlFile, checkedPath: result.path });
      }
    } catch {
      brokenLinks.push({ href, source: relativeHtmlFile, checkedPath: href });
    }
  }
}

if (brokenLinks.length > 0) {
  console.log('✗ Broken links found:\n');
  for (const { href, source, checkedPath } of brokenLinks) {
    console.log(`  ${source}: ${href} (checked: ${checkedPath})`);
  }
  console.log(`\n${brokenLinks.length} broken link(s) found.`);
  process.exit(1);
} else {
  console.log('✓ All internal links are valid.');
}
