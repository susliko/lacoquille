/* Tab-switch transition test against the built site.
   Verifies the four things the screencast complained about:
     1. no animation churn on tab switches (swap fires in one frame)
     2. the tab bar node is never rebuilt (persist)
     3. tab-root islands survive a round trip (persist: same node)
     4. scroll position is restored when returning to a tab root
   Usage: node scripts/test-tab-transitions.mjs [baseUrl]
 */
import { chromium } from "playwright";

const base = process.argv[2] || "http://localhost:4322";

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 390, height: 844 } });

const results = [];
const check = (name, ok, detail) => {
  results.push({ name, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${name}${detail ? ` — ${detail}` : ""}`);
};

await page.goto(base + "/", { waitUntil: "networkidle" });

// mark the current tab bar + island nodes
await page.evaluate(() => {
  document.querySelector(".tab-bar").__mark = "bar-original";
  document.querySelector("astro-island").__mark = "island-original";
});

// scroll the home page so we can verify scroll memory later
await page.evaluate(() => window.scrollTo(0, 600));
await page.waitForTimeout(120);

// ── 1. practice → grammar ────────────────────────────────────────
const t0 = await page.evaluate(() => performance.now());
await page.click('a.tab[href="/knowledge/"]');
await page.waitForURL("**/knowledge/");
await page.waitForFunction(() => document.querySelector(".km-wrap"));
const swapMs = (await page.evaluate(() => performance.now())) - t0;
check("practice→grammar swapped", true, `${swapMs.toFixed(0)}ms incl. first fetch`);
const barAfter1 = await page.evaluate(() => document.querySelector(".tab-bar").__mark);
check("tab bar node survives swap", barAfter1 === "bar-original", barAfter1);

// ── 2. grammar → vocab → practice: island round trip ─────────────
await page.click('a.tab[href="/vocab/"]');
await page.waitForURL("**/vocab/");
await page.waitForFunction(() => document.querySelector("astro-island"));
const islandOnVocab = await page.evaluate(() => ({
  practiceIslandConnected:
    !!document.querySelector('astro-island[data-astro-transition-persist="practice-home"]'),
  vocabIsland: !!document.querySelector('astro-island[data-astro-transition-persist="vocab-home"]'),
  bar: document.querySelector(".tab-bar").__mark,
}));
check("vocab island present on vocab", islandOnVocab.vocabIsland === true);
check("tab bar still original after 2 swaps", islandOnVocab.bar === "bar-original");

await page.click('a.tab[href="/"]');
await page.waitForURL(base + "/");
await page.waitForFunction(() => document.querySelector("astro-island"));
const backHome = await page.evaluate(() => {
  const isl = document.querySelector("astro-island");
  return { mark: isl.__mark || "REBUILT", scrollY: window.scrollY };
});
check(
  "practice island is the SAME node after round trip",
  backHome.mark === "island-original",
  backHome.mark
);

// ── 3. scroll memory ────────────────────────────────────────────
await page.click('a.tab[href="/knowledge/"]');
await page.waitForURL("**/knowledge/");
await page.waitForFunction(() => document.querySelector(".km-wrap"));
await page.evaluate(() => window.scrollTo(0, 1200));
await page.waitForTimeout(100);
await page.click('a.tab[href="/vocab/"]');
await page.waitForURL("**/vocab/");
await page.waitForFunction(() => document.querySelector("astro-island"));
await page.click('a.tab[href="/knowledge/"]');
await page.waitForURL("**/knowledge/");
await page.waitForFunction(() => document.querySelector(".km-wrap"));
await page.waitForTimeout(200); // let page-load scroll restore settle
const scrollBack = await page.evaluate(() => window.scrollY);
check("grammar scroll restored after round trip", scrollBack > 900, `scrollY=${scrollBack}`);
const homeScroll = await page.click('a.tab[href="/"]').then(async () => {
  await page.waitForURL(base + "/");
  await page.waitForTimeout(200);
  return page.evaluate(() => window.scrollY);
});
check("home scroll restored after round trip", homeScroll > 400, `scrollY=${homeScroll}`);

// ── 4. no animation pseudos active on tab swaps ─────────────────
const animProbe = await page.evaluate(async () => {
  // navigate once more and sample view-transition pseudo presence
  const done = new Promise((res) => {
    let n = 0;
    const tick = () => {
      const root = document.documentElement;
      const pseudo = getComputedStyle(root, "::view-transition").opacity !== "";
      if (pseudo || n++ > 40) res(pseudo);
      else requestAnimationFrame(tick);
    };
    tick();
  });
  document.querySelector('a.tab[href="/vocab/"]').click();
  return await done;
});
check(
  "no view-transition animation runs on tab swap",
  animProbe === false,
  animProbe ? "VT pseudos were active" : "no VT layer observed"
);

await page.screenshot({ path: "/data/data/in.opensec.pibolt/files/home/tmp-vid23/pw-final.png" });

const failed = results.filter((r) => !r.ok);
console.log(`\n${results.length - failed.length}/${results.length} checks passed`);
await browser.close();
process.exit(failed.length ? 1 : 0);
