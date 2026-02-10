import React, { useState } from 'react';
import { icons } from '../data';
import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { PenTool, Feather, X } from 'lucide-react';

const DesignInk = () => {
  const [searchTerm, setSearchTerm] = useState('');
  
  const filteredIcons = icons.filter(icon =>
    icon.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#fdfbf7] text-[#1a1a1a] font-serif selection:bg-[#1a1a1a] selection:text-[#fdfbf7] relative overflow-hidden">
      {/* Paper Texture Noise */}
      <div className="fixed inset-0 opacity-[0.03] pointer-events-none z-0" style={{ backgroundImage: `url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E")` }}></div>

      <div className="relative z-10 max-w-5xl mx-auto p-8 border-x border-[#1a1a1a]/10 min-h-screen box-content shadow-[0_0_50px_-20px_rgba(0,0,0,0.1)] bg-[#fdfbf7]">
        
        {/* Header */}
        <header className="mb-16 text-center border-b-4 border-double border-[#1a1a1a] pb-8 pt-4">
          <Link to="/" className="inline-block mb-4 hover:scale-105 transition-transform">
            <div className="border-2 border-[#1a1a1a] rounded-full p-4">
              <Feather size={32} />
            </div>
          </Link>
          <h1 className="text-6xl md:text-7xl font-bold tracking-tighter mb-4" style={{ fontFamily: '"Playfair Display", serif' }}>
            The Icon Press
          </h1>
          <p className="italic text-xl text-[#1a1a1a]/60 font-medium">Est. 2026 • Vector Typography & Symbols</p>
        </header>

        {/* Search */}
        <div className="relative max-w-md mx-auto mb-16 group">
          <input 
            type="text" 
            placeholder="Query the Archive..." 
            className="w-full bg-transparent border-b-2 border-[#1a1a1a] py-3 text-center text-xl focus:outline-none focus:border-[#c0392b] transition-colors placeholder:text-[#1a1a1a]/30 placeholder:italic"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
          <div className="absolute right-0 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity text-[#c0392b]">
            <PenTool size={16} />
          </div>
        </div>

        {/* Grid */}
        <div className="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-5 gap-8 px-4">
          {filteredIcons.map(({ name, Icon }, index) => (
            <motion.button
              key={name}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.01 }}
              onClick={() => navigator.clipboard.writeText(name)}
              className="group flex flex-col items-center gap-4 p-4 hover:bg-[#1a1a1a] hover:text-[#fdfbf7] transition-colors duration-300 relative"
            >
              {/* Corner marks */}
              <div className="absolute top-0 left-0 w-2 h-2 border-t border-l border-[#1a1a1a] group-hover:border-[#fdfbf7]" />
              <div className="absolute top-0 right-0 w-2 h-2 border-t border-r border-[#1a1a1a] group-hover:border-[#fdfbf7]" />
              <div className="absolute bottom-0 left-0 w-2 h-2 border-b border-l border-[#1a1a1a] group-hover:border-[#fdfbf7]" />
              <div className="absolute bottom-0 right-0 w-2 h-2 border-b border-r border-[#1a1a1a] group-hover:border-[#fdfbf7]" />

              <Icon strokeWidth={1.5} size={32} />
              <span className="text-xs uppercase tracking-widest font-semibold">{name}</span>
            </motion.button>
          ))}
        </div>
        
        {/* Footer Stamp */}
        <div className="mt-24 text-center opacity-20 rotate-[-5deg]">
          <div className="border-4 border-[#c0392b] text-[#c0392b] inline-block px-8 py-2 text-2xl font-bold uppercase tracking-widest rounded-sm" style={{ maskImage: 'url("data:image/svg+xml,...")' /* simulation of ink stamp texture */ }}>
            Approved
          </div>
        </div>
      </div>
    </div>
  );
};

export default DesignInk;
