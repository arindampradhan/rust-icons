import * as LucideIcons from 'lucide-react';
import type { LucideProps } from 'lucide-react';

export type IconType = React.ForwardRefExoticComponent<Omit<LucideProps, "ref"> & React.RefAttributes<SVGSVGElement>>;

export const icons = Object.keys(LucideIcons)
  .filter((key) => key !== 'createLucideIcon' && key !== 'default' && key !== 'icons')
  .map((key) => ({
    name: key,
    Icon: (LucideIcons as any)[key] as IconType,
  }))
  // Duplicate the list to simulate density if needed, or just take existing
  // Lucide has ~1000 icons so that should be enough
  .slice(0, 800);
