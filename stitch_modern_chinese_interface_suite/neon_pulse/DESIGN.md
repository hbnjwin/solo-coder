---
name: 酒吧组局平台 Neon Pulse 设计系统
colors:
  surface: '#131313'
  surface-dim: '#131313'
  surface-bright: '#393939'
  surface-container-lowest: '#0e0e0e'
  surface-container-low: '#1b1b1b'
  surface-container: '#1f1f1f'
  surface-container-high: '#2a2a2a'
  surface-container-highest: '#353534'
  on-surface: '#e5e2e1'
  on-surface-variant: '#e6bcbd'
  inverse-surface: '#e5e2e1'
  inverse-on-surface: '#313030'
  outline: '#9f8c8c'
  outline-variant: rgba(230, 188, 189, 0.15)
  surface-tint: '#ffb3b5'
  primary: '#ffdada'
  on-primary: '#512124'
  primary-container: '#ff5167'
  on-primary-container: '#7b4245'
  inverse-primary: '#894e50'
  secondary: '#e8b3ff'
  on-secondary: '#471c5e'
  secondary-container: '#7508a5'
  on-secondary-container: '#d6a2ed'
  tertiary: '#66f7ff'
  on-tertiary: '#003739'
  tertiary-container: '#00dce5'
  on-tertiary-container: '#005c60'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#ffdada'
  primary-fixed-dim: '#ffb3b5'
  on-primary-fixed: '#370c11'
  on-primary-fixed-variant: '#6d373a'
  secondary-fixed: '#f6d9ff'
  secondary-fixed-dim: '#e8b3ff'
  on-secondary-fixed: '#300247'
  on-secondary-fixed-variant: '#603476'
  tertiary-fixed: '#63f7ff'
  tertiary-fixed-dim: '#00dce5'
  on-tertiary-fixed: '#002021'
  on-tertiary-fixed-variant: '#004f53'
  background: '#000000'
  on-background: '#e5e2e1'
  surface-variant: '#353534'
typography:
  headline-lg:
    fontFamily: Space Grotesk
    fontSize: 32px
    fontWeight: '700'
    lineHeight: '1.2'
    letterSpacing: -0.02em
  headline-lg-mobile:
    fontFamily: Space Grotesk
    fontSize: 28px
    fontWeight: '700'
    lineHeight: '1.2'
  title-lg:
    fontFamily: Space Grotesk
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 28px
  body-md:
    fontFamily: Be Vietnam Pro
    fontSize: 16px
    fontWeight: '400'
    lineHeight: 24px
  label-md:
    fontFamily: Be Vietnam Pro
    fontSize: 14px
    fontWeight: '600'
    lineHeight: 20px
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  section-gap: 1.5rem
  container-padding: 1.25rem
  gutter: 1rem
---

```markdown
# Design System Document: Gen-Z Nightlife Social Experience

## 1. Overview & Creative North Star: "The Neon Pulse"
This design system is engineered to capture the kinetic energy of a high-end nightclub. We are moving away from the static, "boxy" nature of standard WeChat Mini Programs toward a **"Neon Pulse"** aesthetic. 

The Creative North Star is **The Digital Curator**: an experience that feels like a VIP backstage pass. We achieve this through "Cyberpunk-lite" aesthetics—combining the raw, pitch-black depth of a dance floor with the hyper-vibrant streaks of light from a DJ booth. By utilizing intentional asymmetry, overlapping glass elements, and high-contrast typography, we create an interface that feels alive, rhythmic, and premium.

---

## 2. Colors & Tonal Depth
The palette is rooted in absolute darkness to allow the neon accents to "bleed" and "glow" with maximum impact.

### Primary Accents & Surface Tokens
- **Background**: `#000000` (Deep Black for maximum OLED contrast)
- **Primary (Neon Pink)**: `primary: #ffb3b5` / `primary_container: #ff5167`
- **Secondary (Electric Purple)**: `secondary: #e8b3ff` / `secondary_container: #7508a5`
- **Tertiary (Cyan/Electric)**: `tertiary: #00dce5` (Used for "Online Now" or "Live" status)

### The "No-Line" Rule
**Strict Prohibition:** 1px solid borders for sectioning are strictly forbidden. 
Boundaries must be defined through:
1.  **Background Color Shifts**: Use `surface_container_low` (#1b1b1b) for a section sitting on a `surface` (#131313) background.
2.  **Tonal Transitions**: Use subtle gradients or the **Glassmorphism Rule** below.

### The "Glass & Gradient" Rule
To create a high-end "Cyberpunk-lite" feel, all floating cards must use:
- **Background**: A semi-transparent `surface_container` with `backdrop-filter: blur(20px)`.
- **Gradients**: Main CTAs should use a linear gradient from `primary` to `primary_container` at a 135-degree angle to simulate light-speed motion.

---

## 3. Typography: Editorial High-Contrast
We use a dual-typeface strategy to blend futuristic tech with premium editorial vibes.

*   **Display & Headlines**: **Space Grotesk**. This font provides the "futuristic/tech" edge. 
    *   *Usage*: Club names, event titles, and bold "Gen-Z" callouts. Use `headline-lg` (2rem) for maximum impact.
*   **Body & Labels**: **Be Vietnam Pro**. A clean, geometric sans-serif that remains legible in low-light (dark mode) environments.
    *   *Usage*: User bios, event descriptions, and UI navigation.

**Chinese Typography Note:** For Simplified Chinese characters, use **PingFang SC** with a "Heavy" weight for headlines and "Regular" for body, ensuring tracking is slightly increased (+2%) for readability against black backgrounds.

---

## 4. Elevation & Depth: Tonal Layering
Traditional drop shadows are too "software-like." We use **Ambient Glows** and **Tonal Layering**.

*   **The Layering Principle**: 
    *   Level 0 (Base): `surface_dim` (#131313)
    *   Level 1 (Sections): `surface_container_low` (#1b1b1b)
    *   Level 2 (Cards): `surface_container` (#1f1f1f) + Glass effect.
*   **Ambient Shadows**: When a card "floats," use a shadow tinted with the `primary` (Pink) or `secondary` (Purple) color at 8% opacity. This mimics the way neon lights reflect off dark surfaces.
*   **The Ghost Border Fallback**: If a container needs more definition against a photograph, use a `outline_variant` border at 15% opacity. Never use 100% opaque lines.

---

## 5. Components

### Buttons (The "Glow" Variant)
*   **Primary**: Rounded (`xl`: 1.5rem), `primary` background, with a soft glow effect (`box-shadow`) matching the button color.
*   **Secondary**: Glassmorphism style. Semi-transparent purple (`secondary_container`) with a white `label-md` text.
*   **States**: On press, the button should scale down to 96% and increase the glow intensity.

### Cards & Lists
*   **Forbid Divider Lines**: Use `spacing-6` (1.5rem) to separate list items. 
*   **Club Cards**: Feature high-quality photography as the background, with a `surface_container_lowest` gradient overlay at the bottom to house the text (`title-lg`).
*   **Roundedness**: All containers must use the `xl` (1.5rem / 24px) corner radius for a friendly yet trendy feel.

### Selection Chips
*   **Style**: Capsule shape (`full`). 
*   **Inactive**: `surface_container_highest` background.
*   **Active**: `primary` (Pink) text with a `primary_container` "ghost" background (20% opacity).

### Inputs
*   **Text Fields**: Pitch black background with a `surface_variant` bottom-only highlight. When focused, the highlight pulses in `secondary` (Purple).

---

## 6. Do’s and Don’ts

### Do:
*   **Use Asymmetry**: Align text to the left but allow images to bleed off the right edge of the screen to create a sense of "endless" content.
*   **Embrace the Glow**: Use the `tertiary` (Cyan) for micro-interactions to provide a "high-tech" feedback loop.
*   **Visual Hierarchy**: Make the "Book Now" (立即预订) or "Join Table" (加入卡座) buttons the only 100% opaque vibrant elements on the screen.

### Don't:
*   **No Pure White Text**: Use `on_surface_variant` (#e6bcbd) for long-form body text to prevent "retinal burn" in dark environments.
*   **No Standard Grids**: Avoid perfectly centered, equal-width boxes. Vary the width of cards (e.g., 60% width for featured, 40% for secondary) to mimic a social media feed.
*   **No Flat Gray Borders**: If it looks like a standard wireframe, it’s wrong. Every edge should be defined by light or color shifts.

---

## 7. Interaction & Motion
*   **Micro-interactions**: When a user swipes a club card, use a subtle parallax effect on the background photography.
*   **Transitions**: All page transitions should use a "fade-in-blur" effect, mimicking the eyes adjusting to a dark room with flashing lights.