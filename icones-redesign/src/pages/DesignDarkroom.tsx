import React, { useState } from 'react';
import { icons } from '../data';
import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Eye, Search, Maximize2 } from 'lucide-react';

const DesignDarkroom = () => {
  const [searchTerm, setSearchTerm] = useState('');
  
  const filteredIcons = icons.filter(icon =>
    icon.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#0a0000] text-[#ff3300] font-mono selection:bg-[#ff3300] selection:text-black">
      {/* Red Safe Light Overlay */}
      <div className="fixed inset-0 pointer-events-none bg-[radial-gradient(circle_at_50%_0%,rgba(255,0,0,0.15),transparent_70%)] z-10" />

      <div className="relative z-20 container mx-auto p-6 md:p-12">
        {/* Header */}
        <header className="flex flex-col md:flex-row justify-between items-end mb-16 border-b border-[#330000] pb-6">
          <div>
            <Link to="/" className="text-3xl font-bold tracking-[0.2em] uppercase text-[#cc0000] hover:text-[#ff0000] transition-colors flex items-center gap-3">
              <Eye className="animate-pulse" /> Darkroom
            </Link>
            <p className="text-[#660000] mt-2 text-sm uppercase tracking-widest">Developing Vector Assets</p>
          </div>
          
          <div className="w-full md:w-auto mt-6 md:mt-0 relative">
             <input 
                type="text" 
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                placeholder="SEARCH_NEGATIVES"
                className="bg-[#1a0000] border border-[#440000] text-[#ff3300] p-3 w-full md:w-64 focus:outline-none focus:border-[#ff0000] focus:shadow-[0_0_15px_rgba(255,0,0,0.3)] placeholder:text-[#440000] uppercase text-sm"
             />
             <Search className="absolute right-3 top-1/2 -translate-y-1/2 text-[#440000]" size={16} />
          </div>
        </header>

        {/* Film Strip Layout */}
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-x-8 gap-y-12">
          {filteredIcons.map(({ name, Icon }) => (
            <motion.div
              key={name}
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              whileHover={{ scale: 1.02 }}
              onClick={() => navigator.clipboard.writeText(name)}
              className="bg-black border-y-8 border-x-8 border-black border-y-[#1a1a1a] relative cursor-pointer group"
              style={{ 
                 // Film Strip Holes Simulation
                 backgroundImage: `
                    radial-gradient(circle, #333 2px, transparent 2.5px),
                    radial-gradient(circle, #333 2px, transparent 2.5px)
                 `,
                 backgroundSize: '100% 20px',
                 backgroundPosition: '0 0, 0 100%',
                 backgroundRepeat: 'repeat-x',
                 paddingTop: '20px',
                 paddingBottom: '20px'
              }}
            >
              <div className="aspect-square bg-[#050000] border border-[#220000] flex flex-col items-center justify-center relative overflow-hidden group-hover:border-[#660000] transition-colors">
                 {/* Inverted "Negative" Look */}
                 <div className="relative z-10 mix-blend-screen text-white opacity-80 group-hover:opacity-100 group-hover:text-[#ff3300] transition-all duration-500">
                    <Icon size={48} strokeWidth={1} />
                 </div>
                 
                 {/* Glow on hover */}
                 <div className="absolute inset-0 bg-[#ff0000] opacity-0 group-hover:opacity-10 blur-xl transition-opacity duration-500" />
                 
                 <div className="absolute bottom-2 left-2 text-[10px] text-[#440000] font-mono group-hover:text-[#ff3300]">
                    ISO 400
                 </div>
                 <div className="absolute bottom-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity text-[#ff3300]">
                    <Maximize2 size={12} />
                 </div>
              </div>

              <div className="mt-2 text-center">
                <span className="text-xs text-[#660000] group-hover:text-[#ff3300] uppercase tracking-widest">{name}</span>
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default DesignDarkroom;
