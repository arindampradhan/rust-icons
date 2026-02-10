import React, { useState } from 'react';
import { icons } from '../data';
import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Triangle, Circle, Square, Search } from 'lucide-react';

const DesignBauhaus = () => {
  const [searchTerm, setSearchTerm] = useState('');
  
  const filteredIcons = icons.filter(icon =>
    icon.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#f0f0e8] text-[#111] font-sans selection:bg-[#e93d3d] selection:text-white overflow-x-hidden">
      
      {/* Decorative Geometric Background */}
      <div className="fixed inset-0 pointer-events-none opacity-20 z-0">
        <div className="absolute top-0 right-0 w-[50vh] h-[50vh] bg-[#e93d3d] rounded-bl-full mix-blend-multiply" />
        <div className="absolute bottom-0 left-0 w-[60vh] h-[60vh] bg-[#1a4b8c] rounded-tr-full mix-blend-multiply" />
        <div className="absolute top-[20%] left-[10%] w-40 h-40 bg-[#f5b700] rounded-full mix-blend-multiply" />
        <div className="absolute top-0 left-1/2 w-2 h-screen bg-[#111] opacity-10" />
      </div>

      <div className="relative z-10 max-w-[1400px] mx-auto p-8 md:p-12">
        {/* Header */}
        <header className="mb-20 grid grid-cols-1 lg:grid-cols-12 gap-12 items-center">
          <div className="lg:col-span-7">
            <Link to="/" className="inline-flex gap-4 mb-8 group">
              <div className="w-12 h-12 bg-[#1a4b8c] rounded-full flex items-center justify-center text-white group-hover:scale-110 transition-transform">
                <Circle size={24} />
              </div>
              <div className="w-12 h-12 bg-[#e93d3d] flex items-center justify-center text-white transform rotate-45 group-hover:rotate-90 transition-transform">
                <Square size={24} />
              </div>
              <div className="w-12 h-12 bg-[#f5b700] flex items-center justify-center text-white group-hover:-translate-y-2 transition-transform">
                <Triangle size={24} />
              </div>
            </Link>
            <h1 className="text-8xl font-black tracking-tighter uppercase leading-[0.8]">
              Form<br />Follows<br /><span className="text-[#e93d3d]">Function</span>
            </h1>
          </div>

          <div className="lg:col-span-5 bg-white p-8 border-4 border-[#111] shadow-[12px_12px_0px_#111]">
            <label className="block text-xl font-bold uppercase mb-4 tracking-widest">Icon Search</label>
            <div className="flex border-4 border-[#111]">
              <div className="bg-[#111] text-white p-4">
                <Search size={24} />
              </div>
              <input 
                type="text"
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-full p-4 text-xl font-bold uppercase focus:outline-none focus:bg-[#f5b700]/10"
                placeholder="TYPE HERE..."
              />
            </div>
          </div>
        </header>

        {/* Content Grid */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8">
          {filteredIcons.map(({ name, Icon }, index) => {
             // Generate random distinctive Bauhaus style per card
             const variant = index % 3;
             return (
              <motion.button
                key={name}
                initial={{ opacity: 0, scale: 0.8 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.4, delay: index * 0.02 }}
                onClick={() => navigator.clipboard.writeText(name)}
                className="group relative aspect-[4/5] bg-white border-4 border-[#111] overflow-hidden hover:shadow-[8px_8px_0px_#e93d3d] transition-all"
              >
                {/* Background Shapes */}
                <div className="absolute inset-0 z-0 opacity-0 group-hover:opacity-20 transition-opacity duration-500">
                  {variant === 0 && <div className="absolute top-0 right-0 w-full h-full bg-[#1a4b8c] rounded-bl-full" />}
                  {variant === 1 && <div className="absolute bottom-0 left-0 w-full h-full bg-[#e93d3d] clip-path-polygon" style={{ clipPath: 'polygon(0 0, 100% 100%, 0 100%)' }} />}
                  {variant === 2 && <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[120%] h-[120%] bg-[#f5b700] rounded-full" />}
                </div>

                <div className="relative z-10 h-full flex flex-col">
                  <div className="flex-1 flex items-center justify-center p-8">
                    <Icon size={64} strokeWidth={1} className="group-hover:scale-125 transition-transform duration-500" />
                  </div>
                  
                  <div className="border-t-4 border-[#111] p-4 bg-white flex justify-between items-center">
                    <span className="font-bold uppercase tracking-wider truncate">{name}</span>
                    <div className={`w-3 h-3 rounded-full ${variant === 0 ? 'bg-[#1a4b8c]' : variant === 1 ? 'bg-[#e93d3d]' : 'bg-[#f5b700]'}`} />
                  </div>
                </div>
              </motion.button>
            );
          })}
        </div>
      </div>
    </div>
  );
};

export default DesignBauhaus;
