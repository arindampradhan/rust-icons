import React, { useState } from 'react';
import { icons } from '../data';
import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Search, Candy as CandyIcon } from 'lucide-react';

const DesignCandy = () => {
  const [searchTerm, setSearchTerm] = useState('');
  
  const filteredIcons = icons.filter(icon =>
    icon.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#f0f8ff] text-[#5c5c8a] font-sans selection:bg-[#ff69b4] selection:text-white p-6">
      
      {/* Bubbles Background */}
      <div className="fixed inset-0 pointer-events-none overflow-hidden z-0">
         <div className="absolute top-[-10%] left-[10%] w-96 h-96 bg-purple-200 rounded-full mix-blend-multiply filter blur-3xl opacity-50 animate-bounce duration-[10s]" />
         <div className="absolute top-[20%] right-[-10%] w-96 h-96 bg-pink-200 rounded-full mix-blend-multiply filter blur-3xl opacity-50 animate-bounce duration-[12s] delay-1000" />
         <div className="absolute bottom-[-10%] left-[30%] w-96 h-96 bg-blue-200 rounded-full mix-blend-multiply filter blur-3xl opacity-50 animate-bounce duration-[15s] delay-2000" />
      </div>

      <div className="relative z-10 max-w-7xl mx-auto">
        {/* Floating Navbar */}
        <nav className="bg-white/70 backdrop-blur-xl border border-white/50 shadow-[0_8px_32px_rgba(0,0,0,0.05)] rounded-[2rem] p-4 flex flex-col md:flex-row items-center justify-between mb-12 sticky top-4">
          <Link to="/" className="flex items-center gap-3 px-4 py-2 hover:scale-105 transition-transform">
             <div className="w-10 h-10 bg-gradient-to-br from-pink-400 to-purple-500 rounded-2xl flex items-center justify-center text-white shadow-lg shadow-pink-200">
               <CandyIcon size={20} />
             </div>
             <span className="text-2xl font-black bg-clip-text text-transparent bg-gradient-to-r from-pink-500 to-purple-600 tracking-tight">
               SweetIcons
             </span>
          </Link>

          <div className="relative w-full md:w-96 group">
            <input 
               type="text" 
               placeholder="Find a treat..." 
               value={searchTerm}
               onChange={(e) => setSearchTerm(e.target.value)}
               className="w-full bg-slate-50 border-2 border-transparent focus:border-purple-200 focus:bg-white rounded-2xl py-3 pl-12 pr-4 text-slate-600 placeholder:text-slate-400 transition-all shadow-inner focus:shadow-lg focus:shadow-purple-100/50 outline-none font-bold"
            />
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 text-slate-400 group-focus-within:text-purple-400 transition-colors" />
          </div>
        </nav>

        {/* Icons Grid */}
        <div className="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-6">
          {filteredIcons.map(({ name, Icon }, index) => (
             <motion.button
               key={name}
               initial={{ scale: 0, opacity: 0 }}
               animate={{ scale: 1, opacity: 1 }}
               transition={{ type: "spring", bounce: 0.5, delay: index * 0.03 }}
               whileHover={{ y: -8, scale: 1.05 }}
               whileTap={{ scale: 0.95 }}
               onClick={() => navigator.clipboard.writeText(name)}
               className="group flex flex-col items-center gap-3"
             >
               <div className="w-full aspect-square bg-white rounded-[2rem] shadow-[0_10px_20px_rgba(0,0,0,0.03)] border border-white flex items-center justify-center relative overflow-hidden group-hover:shadow-[0_20px_40px_rgba(168,85,247,0.15)] transition-all duration-300">
                  {/* Glossy sheen */}
                  <div className="absolute top-0 left-0 w-full h-1/2 bg-gradient-to-b from-white/80 to-transparent opacity-50 rounded-t-[2rem]" />
                  
                  <div className="relative z-10 text-slate-400 group-hover:text-purple-500 transition-colors duration-300">
                    <Icon size={32} strokeWidth={2} className="drop-shadow-sm" />
                  </div>
               </div>
               
               <span className="text-xs font-bold text-slate-400 group-hover:text-purple-500 transition-colors truncate w-full text-center px-2">
                 {name}
               </span>
             </motion.button>
          ))}
        </div>
      </div>
    </div>
  );
};

export default DesignCandy;
