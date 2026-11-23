/* Tab bar behaviour test against the dev or built site.
   Verifies the nav bar's final form — press feedback only, activation
   is motionless (no background indicator, no icon animation):
     1. no background indicator, no choreography classes on load
     2. icon variants swap (outline ↔ filled) with the .on class
     3. activation adds no animation to the icon (twitch regression)
     4. active ink is navy, inactive is muted
     5. held tab compresses to 0.95 and springs back on release
     6. reduced motion: variant swap survives (it's not animated anyway)
     7. no native WebView tap highlight on tabs (grey blob twitch)
     8. :hover restyles inactive tabs on mouse, never on touch
   Usage: node scripts/test-tab-activation.mjs [baseUrl]
 */
import { chromium } from "playwright";

const base = process.argv[2] || "http://localhost:4321";

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 390, height: 844 } });

const results = [];
const check = (name, ok, detail) => {
  results.push(ok);
  console.log(`${ok ? "PASS" : "FAIL"}  ${name}${detail ? ` — ${detail}` : ""}`);
};

// ── 1. cold load: clean bar, outline icon on inactive ──
await page.goto(base + "/", { waitUntil: "networkidle" });
const cold = await page.evaluate(() => {
  const bar = document.querySelector(".tab-bar");
  const home = bar.querySelectorAll("a.tab")[0];
  const practice = bar.querySelectorAll("a.tab")[1];
  return {
    noPill: !bar.querySelector(".tab-pill"),
    noPopClass: !document.querySelector(".tab-bar .pop"),
    tapHighlight: getComputedStyle(home).webkitTapHighlightColor,
    homeFill: getComputedStyle(home.querySelector(".ic-f")).display,
    practiceFill: getComputedStyle(practice.querySelector(".ic-f")).display,
  };
});
check("no background indicator in the bar", cold.noPill === true);
check("no .pop class anywhere", cold.noPopClass === true);
check("no native tap highlight on tabs", cold.tapHighlight === "rgba(0, 0, 0, 0)", cold.tapHighlight);
check("inactive icon is outline (filled hidden)", cold.practiceFill === "none");
check("active icon is filled (shown)", cold.homeFill === "block");

// ── 2. tap Practice → variant swaps, NO icon animation ──
await page.click('a.tab[href="/practice/"]');
await page.waitForFunction(() =>
  document.querySelector(".tab-bar a.tab.on")?.getAttribute("href") === "/practice/"
);
const mid = await page.evaluate(() => {
  const bar = document.querySelector(".tab-bar");
  const practice = bar.querySelectorAll("a.tab")[1];
  const cs = getComputedStyle(practice.querySelector(".ic"));
  return {
    fill: getComputedStyle(practice.querySelector(".ic-f")).display,
    outline: getComputedStyle(practice.querySelector(".ic-o")).display,
    color: getComputedStyle(practice).color,
    animName: cs.animationName,
    animDur: cs.animationDuration,
    popClass: practice.classList.contains("pop"),
    homeFill: getComputedStyle(bar.querySelectorAll("a.tab")[0].querySelector(".ic-o")).display,
  };
});
check("active icon filled on arrival", mid.fill === "block");
check("old tab back to outline", mid.homeFill === "none");
check("icon has NO animation (no twitch)", mid.animName === "none" || mid.animDur === "0s", `${mid.animName} ${mid.animDur}`);
check("no .pop class after swap", mid.popClass === false);
check("active ink is navy", mid.color === "rgb(21, 56, 111)", mid.color);

// ── 3. flip back home — same stillness ──
await page.click('a.tab[href="/"]');
await page.waitForFunction(() =>
  document.querySelector(".tab-bar a.tab.on")?.getAttribute("href") === "/"
);
const back = await page.evaluate(() => {
  const home = document.querySelectorAll(".tab-bar a.tab")[0];
  const cs = getComputedStyle(home.querySelector(".ic"));
  return {
    fill: getComputedStyle(home.querySelector(".ic-f")).display,
    animName: cs.animationName,
    animDur: cs.animationDuration,
  };
});
check("flip home: filled icon, no animation", back.fill === "block" && (back.animName === "none" || back.animDur === "0s"));

// ── 4. press: compression on the tab, ripple elsewhere but not on it ──
const held = page.locator('a.tab[href="/practice/"]');
const box = await held.boundingBox();
await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
await page.mouse.down();
await page.waitForTimeout(150);
const pressState = await page.evaluate(() => ({
  transform: getComputedStyle(document.querySelectorAll(".tab-bar a.tab")[1]).transform,
  veil: getComputedStyle(document.querySelectorAll(".tab-bar a.tab")[1], "::after").opacity,
  rippleInBar: !!document.querySelector(".tab-bar .ripple"),
}));
await page.mouse.up();
check("held tab compresses (scale 0.95)", pressState.transform.includes("0.95"), pressState.transform);
check("state layer veil on while held (0.12)", pressState.veil === "0.12", pressState.veil);
check("no ripple span on the tab", pressState.rippleInBar === false);
await page.waitForTimeout(350);
const releaseState = await page.evaluate(() => ({
  transform: getComputedStyle(document.querySelectorAll(".tab-bar a.tab")[1]).transform,
  veil: getComputedStyle(document.querySelectorAll(".tab-bar a.tab")[1], "::after").opacity,
}));
check("release springs back", releaseState.transform === "none" || releaseState.transform.includes("matrix(1, 0, 0, 1"), releaseState.transform);
check("veil dissolves on release", parseFloat(releaseState.veil) < 0.02, releaseState.veil);

// grid cells press with the state layer too (no spans, pure CSS)
await page.goto(base + "/practice/", { waitUntil: "networkidle" });
const cell = page.locator(".gg-cell").first();
const cbox = await cell.boundingBox();
await page.mouse.move(cbox.x + cbox.width / 2, cbox.y + cbox.height / 2);
await page.mouse.down();
await page.waitForTimeout(150);
const cellPress = await page.evaluate(() => ({
  veil: getComputedStyle(document.querySelector(".gg-cell"), "::after").opacity,
  transform: getComputedStyle(document.querySelector(".gg-cell")).transform,
  span: !!document.querySelector(".gg-cell .ripple"),
}));
await page.mouse.up();
check("grid cell veil on press (0.12)", cellPress.veil === "0.12", cellPress.veil);
check("grid cell compresses (0.97)", cellPress.transform.includes("0.97"), cellPress.transform);
check("no ripple span on the cell", cellPress.span === false);

// ── 5. reduced motion: nothing to collapse, variant swap survives ──
const rmPage = await browser.newPage({ viewport: { width: 390, height: 844 }, reducedMotion: "reduce" });
await rmPage.goto(base + "/", { waitUntil: "networkidle" });
await rmPage.click('a.tab[href="/practice/"]');
await rmPage.waitForFunction(() =>
  document.querySelector(".tab-bar a.tab.on")?.getAttribute("href") === "/practice/"
);
const rm = await rmPage.evaluate(
  () => getComputedStyle(document.querySelectorAll(".tab-bar a.tab")[1].querySelector(".ic-f")).display,
);
check("reduced motion: variant swap still instant", rm === "block");
await rmPage.close();

// ── 8. hover: mouse restyles, touch doesn't latch ──
const mousePage = await browser.newPage({ viewport: { width: 390, height: 844 } });
await mousePage.goto(base + "/", { waitUntil: "networkidle" });
await mousePage.hover('a.tab[href="/practice/"]');
const mouseHover = await mousePage.evaluate(
  () => getComputedStyle(document.querySelectorAll(".tab-bar a.tab")[1]).color,
);
check("mouse: hover inks the inactive tab", mouseHover === "rgb(31, 29, 24)", mouseHover);
await mousePage.close();
const touchPage = await browser.newPage({ viewport: { width: 390, height: 844 }, hasTouch: true });
await touchPage.goto(base + "/", { waitUntil: "networkidle" });
await touchPage.hover('a.tab[href="/practice/"]');
const touchHover = await touchPage.evaluate(
  () => getComputedStyle(document.querySelectorAll(".tab-bar a.tab")[1]).color,
);
check("touch: hover does NOT restyle the tab", touchHover === "rgb(91, 97, 110)", touchHover);
await touchPage.close();

await browser.close();
const failed = results.filter((r) => !r).length;
console.log(`\n${results.length - failed}/${results.length} passed`);
process.exit(failed ? 1 : 0);
