import { useState } from 'react';
import { icons } from '../data';
import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Search, Coffee, Grid, List } from 'lucide-react';

const CollectionHome = () => {
  const [searchTerm, setSearchTerm] = useState('');
  
  // Categorized icons mock
  const categories = {
    'Recent': icons.slice(0, 4),
    'Material': icons.slice(4, 8),
    'UI 24px': icons.slice(8, 20),
  };

  return (
    <div className="min-h-screen bg-[#f1f0e8] text-[#1a1a1a] font-serif overflow-x-hidden p-2 md:p-8">
      <div className="max-w-7xl mx-auto bg-[#fbfbf8] shadow-2xl min-h-screen border-x border-black/10 relative">
        
        {/* Header Section */}
        <header className="border-b-4 border-black p-8 text-center bg-white sticky top-0 z-20">
          <div className="flex justify-between items-center border-b border-black pb-2 mb-4 text-xs font-sans font-bold uppercase tracking-widest">
            <span>Vol. CCLVI No. 104</span>
            <span className="flex items-center gap-2">Icônes <Coffee size={12}/></span>
            <span>$4.00</span>
          </div>
          
          <div className="relative mb-6">
             <input 
                type="text" 
                placeholder="Search category..." 
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-full bg-transparent border-2 border-black p-3 font-serif text-lg placeholder:italic focus:outline-none focus:ring-1 focus:ring-black"
             />
             <Search className="absolute right-4 top-1/2 -translate-y-1/2 text-black" size={20} />
          </div>

          <div className="border-y-2 border-black py-2 flex justify-center gap-8 text-sm font-sans font-bold uppercase overflow-x-auto">
             <span className="hover:underline cursor-pointer whitespace-nowrap">All</span>
             <span className="hover:underline cursor-pointer whitespace-nowrap">Recent</span>
             <span className="hover:underline cursor-pointer whitespace-nowrap">Material</span>
             <span className="hover:underline cursor-pointer whitespace-nowrap">UI 24px</span>
             <span className="hover:underline cursor-pointer whitespace-nowrap">Logos</span>
             <span className="hover:underline cursor-pointer whitespace-nowrap">Emoji</span>
          </div>
        </header>

        <div className="p-8 space-y-12">
           {/* Sections */}
           {Object.entries(categories).map(([category, items]) => (
              <section key={category}>
                 <div className="flex items-baseline gap-4 mb-6 border-b-2 border-black pb-2">
                    <h2 className="text-3xl font-black font-serif tracking-tight">{category}</h2>
                    <span className="text-sm font-sans text-gray-500 uppercase tracking-widest">Section {category[0]}</span>
                 </div>
                 
                 <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    {items.map((icon, i) => (
                       <Link to={`/collection/${icon.name}`} key={icon.name} className="group border border-gray-300 bg-white p-4 hover:shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:border-black transition-all">
                          <div className="flex justify-between items-start mb-4">
                             <h3 className="font-bold text-lg font-serif leading-tight group-hover:text-[#b91c1c] transition-colors">{icon.name}</h3>
                             <div className="text-[10px] font-sans bg-gray-100 px-1 border border-gray-200">SVG</div>
                          </div>
                          
                          <div className="aspect-video bg-gray-50 border border-gray-100 mb-3 flex items-center justify-center relative overflow-hidden">
                             <div className="absolute inset-0 opacity-10" style={{ backgroundImage: 'radial-gradient(#000 1px, transparent 1px)', backgroundSize: '10px 10px' }}></div>
                             <icon.Icon size={32} strokeWidth={1} />
                          </div>
                          
                          <div className="flex justify-between items-end text-xs text-gray-500 font-sans border-t border-gray-100 pt-2">
                             <span>{Math.floor(Math.random() * 5000)} icons</span>
                             <span className="uppercase tracking-widest">MIT</span>
                          </div>
                       </Link>
                    ))}
                 </div>
              </section>
           ))}
        </div>
      </div>
    </div>
  );
};

export default CollectionHome;
