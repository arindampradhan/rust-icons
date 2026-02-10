# Design System: Rust Icons

This design system is inspired by traditional newspaper aesthetics, focusing on typography, structured layouts, and a clean, high-contrast reading experience.

## 1. Core Principles
- **Authority**: Uses serif fonts and structured grids to convey reliability.
- **Hierarchy**: Clear distinction between headlines, subheads, and body text.
- **Density**: Maximizes information density while maintaining readability through columns and dividers.
- **Monochrome with Accent**: Primarily black and white/cream, with subtle gray accents.

## 2. Color Palette

### Primary
- **Paper Background**: `#f1f0e8` (Outer background)
- **Newsprint Background**: `#fbfbf8` (Main content area)
- **Ink Black**: `#1a1a1a` (Primary text)
- **Stroke Black**: `#000000` (Borders, dividers)

### Secondary
- **Gray Accent**: `#e5e7eb` (bg-gray-200) - Used for image placeholders/backgrounds.
- **Subtext**: `#4b5563` (text-gray-600) - Used for captions, dates, and meta info.
- **Border Gray**: `#e5e7eb` (border-gray-200) - Subtle dividers.
- **Interaction**: `#b91c1c` (text-red-700) - Hover state for headlines.

## 3. Typography

### Font Families
- **Serif (Headlines, Body)**: `font-serif` (Times New Roman, Garamond, etc.) - The voice of the paper.
- **Sans-Serif (Meta, UI Elements)**: `font-sans` (Arial, Helvetica, etc.) - Used for navigation, prices, weather, and small utility text.

### Type Scale
- **Masthead**: `text-6xl` to `text-8xl`, `font-black`, `tracking-tight`
- **Headline (Featured)**: `text-xl`, `font-bold`
- **Headline (Standard)**: `text-sm`, `font-bold`
- **Body**: `text-sm`, `leading-relaxed`
- **Meta / Utility**: `text-xs`, `uppercase`, `tracking-widest`

## 4. Components

### 4.1. Header / Masthead
- **Structure**: Centered, bordered top and bottom.
- **Meta Bar**: Top border-bottom containing volume, date, price. Sans-serif, uppercase, bold.
- **Title**: Massive serif font.
- **Motto**: Italicized serif.
- **Navigation**: Border-top and bottom. Uppercase, sans-serif, bold.

### 4.2. Cards / Articles
- **Featured Article**:
  - Full width in column.
  - Border-bottom divider.
  - Large image placeholder (aspect-video).
  - Large headline.
  - 3-line clamped body text.
- **Standard Entry**:
  - Row layout (Icon left, Text right).
  - Small icon container.
  - Small headline.
  - Meta info (file type, size).
  - Border-bottom divider.

### 4.3. Search Bar
- **Style**: Minimalist.
- **Border**: Bottom-only (`border-b-2`).
- **Input**: Serif font, large text (`text-2xl`), italic placeholder.

### 4.4. Sidebar
- **Purpose**: Weather, Index, Ads.
- **Style**: Right border divider (`border-r`).
- **Headers**: Sans-serif, uppercase, bold, border-bottom.

## 5. Layout & Spacing
- **Container**: Max width `6xl`, centered, shadow-2xl.
- **Grid System**: 12-column grid (`grid-cols-12`) for main structure.
  - Sidebar: 3 columns (hidden on mobile).
  - Main Content: 9 columns.
- **Editorial Grid**: Masonry-style using CSS columns (`columns-1`, `sm:columns-2`, `lg:columns-3`).
- **Borders**: Heavy use of `border-b`, `border-r` to define spaces without background colors.

## 6. Icons & Assets
- **Icon Style**: Lucide React icons.
- **Stroke Width**: `1px` (Fine/Thin) matches the detailed serif aesthetic.
- **Sizes**:
  - `size={48}` for Featured.
  - `size={24}` for Standard.
  - `size={12}` for Tiny/Meta.

## 7. Interaction States
- **Hover**:
  - Headlines turn red (`text-red-700`).
  - Images/Cards opacity change or underline.
  - Cursor pointer.

## 8. CSS Classes (Tailwind Reference)

```css
/* Masthead */
.masthead {
  @apply text-6xl md:text-8xl font-black font-serif tracking-tight mb-2;
}

/* Section Header */
.section-header {
  @apply font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1;
}

/* Article Headline */
.headline {
  @apply font-bold font-serif leading-tight group-hover:text-red-700 transition-colors;
}

/* Body Text */
.body-text {
  @apply text-sm text-gray-600 font-serif leading-relaxed;
}

/* Paper Container */
.paper-container {
  @apply bg-[#fbfbf8] shadow-2xl border-x border-black/10;
}
```
