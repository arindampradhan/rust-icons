import React, { useState } from 'react';
import { icons } from '../data';
import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Terminal, Command, ChevronRight } from 'lucide-react';

const DesignTerminal = () => {
  const [searchTerm, setSearchTerm] = useState('');
  
  const filteredIcons = icons.filter(icon =>
    icon.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#0c0c0c] text-[#33ff00] font-mono selection:bg-[#33ff00] selection:text-black p-4 md:p-8">
      <div className="max-w-7xl mx-auto border-2 border-[#33ff00] rounded-lg min-h-[90vh] flex flex-col shadow-[0_0_20px_rgba(51,255,0,0.15)] bg-black relative overflow-hidden">
        
        {/* Scanlines */}
        <div className="absolute inset-0 pointer-events-none bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.1)_50%)] bg-[length:100%_4px] z-20 opacity-30"></div>
        <div className="absolute inset-0 pointer-events-none bg-[radial-gradient(circle,rgba(51,255,0,0.05),transparent_80%)] z-10"></div>

        {/* Title Bar */}
        <header className="bg-[#33ff00] text-black px-4 py-2 font-bold flex justify-between items-center uppercase tracking-wider shrink-0 z-30">
          <div className="flex items-center gap-2">
            <Terminal size={16} fill="currentColor" />
            <span>BASH // ICON_EXPLORER.EXE</span>
          </div>
          <Link to="/" className="hover:bg-black hover:text-[#33ff00] px-2 py-0.5 rounded-[2px] transition-colors">
             [X] EXIT
          </Link>
        </header>

        {/* Content */}
        <div className="p-6 flex-1 flex flex-col overflow-hidden relative z-30">
          
          <div className="mb-6 space-y-2 text-sm opacity-80">
            <p>&gt; INITIALIZING SYSTEM...</p>
            <p>&gt; LOADING MODULES... DONE</p>
            <p>&gt; CONNECTED TO LOCALHOST:5173</p>
            <p>&gt; {filteredIcons.length} OBJECTS FOUND IN MEMORY.</p>
          </div>

          {/* Input Line */}
          <div className="flex items-center gap-3 border-b border-[#33ff00]/30 pb-4 mb-6 text-lg">
            <span className="animate-pulse">_</span>
            <span className="text-[#33ff00]">root@user:~$</span>
            <span className="text-white">grep</span>
            <input 
              type="text" 
              className="bg-transparent border-none focus:outline-none text-[#33ff00] w-full caret-[#33ff00] uppercase"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              autoFocus
              placeholder="SEARCH_PATTERN"
            />
          </div>

          {/* Grid */}
          <div className="flex-1 overflow-y-auto custom-scrollbar">
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
              {filteredIcons.map(({ name, Icon }) => (
                <button
                  key={name}
                  onClick={() => navigator.clipboard.writeText(name)}
                  className="group flex items-center gap-4 p-3 border border-[#33ff00]/20 hover:bg-[#33ff00] hover:text-black transition-all text-left font-bold"
                >
                  <div className="border-r border-[#33ff00]/50 pr-4 group-hover:border-black">
                    <Icon size={20} />
                  </div>
                  <div className="flex flex-col overflow-hidden">
                    <span className="truncate text-xs uppercase tracking-wider">{name}</span>
                    <span className="text-[10px] opacity-50 group-hover:opacity-80">SVG_ID: {Math.floor(Math.random() * 9999)}</span>
                  </div>
                </button>
              ))}
            </div>
          </div>

        </div>

        {/* Status Bar */}
        <footer className="border-t border-[#33ff00] p-2 flex justify-between text-xs uppercase z-30 bg-black">
           <span>MEM: 640K OK</span>
           <span className="animate-pulse">CURSOR_ACTIVE</span>
        </footer>
      </div>
      
      <style>{`
        .custom-scrollbar::-webkit-scrollbar {
          width: 12px;
        }
        .custom-scrollbar::-webkit-scrollbar-track {
          background: #001100;
          border-left: 1px solid #33ff00;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb {
          background: #33ff00;
          border: 2px solid #000;
        }
      `}</style>
    </div>
  );
};

export default DesignTerminal;
