---
name: oklch-colors
description: 'OKLCH color space reference for CSS. Use when: choosing colors, building palettes, converting hex/rgb/hsl to oklch, checking perceived lightness, creating accessible color scales, working with P3 or wide-gamut colors.'
---

OKLCH captures perceived Lightness, Chroma, Hue, and Alpha. Values are human-readable and differences are predictable just by reading them. It represents wide-gamut colors (P3, Rec2020) that HSL and RGB cannot.

OKLCH improves on LCH by fixing a bug in the blue chroma range and providing better [gamut correction](https://bottosson.github.io/posts/gamutclipping/).

## Syntax

```css
.c { color: oklch(0.8 0.12 100 / 100%); }
/*                ^   ^    ^     ^
                  |   |    |     |
          lightness   |    |     |
               chroma |    |     |
                     hue   |     |
                       alpha     */
```

- **Lightness** `0–1`: Perceived brightness. `0` = black, `1` = white. Equal steps look equal.
- **Chroma** `0–0.4`: Color intensity. `0` = gray, higher = more vivid. Most usable colors sit between `0.05–0.2`.
- **Hue** `0–360`: Color wheel angle. `0` = pink/red, `90` = yellow, `150` = green, `225` = blue, `300` = purple.
- **Alpha** `0%–100%`: Opacity. Optional, defaults to 100%.

## Quick Reference

```css
.bw {
  color: oklch(0 0 0);       /* black */
  color: oklch(1 0 0);       /* white */
  color: oklch(0.5 0 0);     /* gray (any hue at chroma 0 is gray) */
}
.colors {
  color: oklch(0.8 0.12 100);  /* yellow */
  color: oklch(0.6 0.12 100);  /* darker yellow — only lightness changed */
  color: oklch(0.8 0.05 100);  /* grayish yellow — only chroma changed */
  color: oklch(0.8 0.12 225);  /* blue, same perceived lightness as the yellow */
}
.opacity {
  color: oklch(0.8 0.12 100 / 50%);  /* 50% transparent yellow */
}
```

## Key Advantages

- **Perceptually uniform**: Equal lightness steps *look* equal. Two colors at `L=0.7` appear equally bright regardless of hue — not true in HSL.
- **Predictable manipulation**: Darken by subtracting from L, desaturate by reducing C, shift hue by changing H. Each axis is independent.
- **Wide gamut**: Represents P3 and Rec2020 colors that sRGB hex/rgb cannot.
- **Better than LCH**: Fixes the blue hue shift bug present in CIE LCH.

## Building Accessible Scales

To create a color scale with guaranteed contrast:

```css
/* Same hue (250 = violet), decreasing lightness */
--violet-100: oklch(0.95 0.03 250);  /* lightest, backgrounds */
--violet-300: oklch(0.80 0.08 250);  /* borders, subtle accents */
--violet-500: oklch(0.65 0.15 250);  /* primary accent */
--violet-700: oklch(0.45 0.15 250);  /* dark foreground on light bg */
--violet-900: oklch(0.25 0.10 250);  /* darkest, text on light bg */
```

Each 0.15 lightness step gives roughly 3:1 contrast between adjacent levels.

## Common Patterns

**Tinted neutrals** — warm or cool grays with a subtle hue:
```css
--neutral-warm: oklch(0.95 0.01 60);   /* warm off-white */
--neutral-cool: oklch(0.95 0.01 250);  /* cool off-white */
```

**Semantic colors** — consistent lightness for equal visual weight:
```css
--success: oklch(0.72 0.17 145);  /* green */
--warning: oklch(0.80 0.16 75);   /* amber */
--error:   oklch(0.65 0.20 25);   /* red */
--info:    oklch(0.70 0.12 240);  /* blue */
```

**Dark mode counterparts** — same hue/chroma, higher lightness:
```css
/* Light mode accent */
--accent: oklch(0.55 0.20 285);
/* Dark mode — raise lightness to maintain contrast on dark bg */
--accent: oklch(0.75 0.15 285);
```

## Tooling

- **Color picker**: [oklch.com](https://oklch.com)
- **Palette generator**: [huetone.ardov.me](https://huetone.ardov.me)
- **Figma plugin**: [OkColor](https://www.figma.com/community/plugin/1173638098109123591/okcolor)
- **Batch conversion**: `npx convert-to-oklch ./src/**/*.css`
