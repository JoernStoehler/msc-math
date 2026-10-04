/* File-based source checks: relative URLs keep every read in this checkout. */
window.dashboardSourceStatus = async function () {
  if (location.pathname.startsWith('/docs/dashboard/')) {
    // Preserve the existing dedicated dashboard server's check on plain HTTP.
    const response = await fetch('/_dashboard/source-status', {cache: 'no-store'});
    if (!response.ok) throw new Error('Source check unavailable');
    return response.json();
  }
  if (!globalThis.crypto?.subtle) return {known: false, reconciled: false};
  const response = await fetch(new URL('thesis-sources.json', location.href), {cache: 'no-store'});
  if (!response.ok) throw new Error('Source baseline unavailable');
  const baseline = await response.json();
  if (typeof baseline.reconciled !== 'boolean' || typeof baseline.note !== 'string' ||
      !baseline.sha256 || typeof baseline.sha256 !== 'object') throw new Error('Invalid source baseline');
  const root = new URL('../../', location.href);
  const changed = [];
  for (const [path, expected] of Object.entries(baseline.sha256)) {
    if (!path || path.startsWith('/') || path.includes('..') || path.includes(':') ||
        path.includes('\\') || !/^[a-f0-9]{64}$/.test(expected)) throw new Error('Invalid source path/hash');
    const url = new URL(path, root);
    url.searchParams.set('raw', '1');
    const source = await fetch(url, {cache: 'no-store'});
    if (!source.ok) throw new Error('Source unavailable');
    const digest = await crypto.subtle.digest('SHA-256', await source.arrayBuffer());
    const actual = Array.from(new Uint8Array(digest), byte => byte.toString(16).padStart(2, '0')).join('');
    if (actual !== expected) changed.push(path);
  }
  return {known: true, changed, reconciled: baseline.reconciled, note: baseline.note};
};
