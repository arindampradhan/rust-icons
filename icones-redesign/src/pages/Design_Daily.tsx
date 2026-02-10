import { useState } from 'react';
import { icons } from '../data';
import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Newspaper, Search, Coffee } from 'lucide-react';

const Design_Daily = () => {
  const [searchTerm, setSearchTerm] = useState('');
  
  const filteredIcons = icons.filter(icon =>
    icon.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#f1f0e8] text-[#1a1a1a] font-serif overflow-x-hidden p-2 md:p-8">
      <div className="max-w-6xl mx-auto bg-[#fbfbf8] shadow-2xl min-h-screen border-x border-black/10 relative">
        
        {/* Header Section */}
        <header className="border-b-4 border-black p-8 text-center">
          <div className="flex justify-between items-center border-b border-black pb-2 mb-4 text-xs font-sans font-bold uppercase tracking-widest">
            <span>Vol. CCLVI No. 104</span>
            <span className="flex items-center gap-2">Since 2026 <Coffee size={12}/></span>
            <span>$4.00</span>
          </div>
          
          <Link to="/" className="block">
            <h1 className="text-6xl md:text-8xl font-black font-serif tracking-tight mb-2 hover:opacity-80 transition-opacity">
              The Daily Icon
            </h1>
          </Link>
          
          <div className="italic text-lg text-gray-600 mb-6 font-serif">
            "All the Vectors That Are Fit to Print"
          </div>

          <div className="border-y-2 border-black py-2 flex justify-center gap-8 text-sm font-sans font-bold uppercase">
             <span className="hover:underline cursor-pointer">World</span>
             <span className="hover:underline cursor-pointer">Business</span>
             <span className="hover:underline cursor-pointer">Tech</span>
             <span className="hover:underline cursor-pointer">Arts</span>
             <span className="hover:underline cursor-pointer">Opinion</span>
          </div>
        </header>

        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 p-8">
           {/* Sidebar / Left Column */}
           <aside className="lg:col-span-3 border-r border-black/20 pr-8 hidden lg:block">
              <h3 className="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">Weather</h3>
              <div className="mb-8 text-sm font-serif">
                 <p>Sunny with a chance of vectors.</p>
                 <p className="font-bold">High 72°F</p>
              </div>

              <h3 className="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">Index</h3>
              <ul className="text-sm font-serif space-y-2">
                 <li className="flex justify-between"><span>Arrows</span> <span>A3</span></li>
                 <li className="flex justify-between"><span>Media</span> <span>B1</span></li>
                 <li className="flex justify-between"><span>System</span> <span>C4</span></li>
                 <li className="flex justify-between"><span>Weather</span> <span>D2</span></li>
              </ul>
              
              <div className="mt-12 bg-black text-white p-4 text-center">
                 <h4 className="font-sans font-bold uppercase text-xs mb-2">Advertisement</h4>
                 <div className="border border-white/30 p-4 font-serif italic text-sm">
                    "Got SVG?"
                 </div>
              </div>
           </aside>

           {/* Main Content */}
           <main className="lg:col-span-9">
              {/* Search Headline */}
              <div className="mb-8 text-center">
                 <div className="inline-flex items-center border-b-2 border-black w-full max-w-xl pb-2 gap-4">
                    <Search className="text-black" />
                    <input 
                       type="text" 
                       placeholder="Search the archives..." 
                       value={searchTerm}
                       onChange={(e) => setSearchTerm(e.target.value)}
                       className="w-full bg-transparent font-serif text-2xl placeholder:italic focus:outline-none placeholder:text-gray-400"
                    />
                 </div>
              </div>

              {/* Editorial Grid */}
              <div className="columns-1 sm:columns-2 lg:columns-3 gap-8 space-y-8">
                 {filteredIcons.map(({ name, Icon }, index) => (
                    <div key={name} className="break-inside-avoid mb-8 cursor-pointer group" onClick={() => navigator.clipboard.writeText(name)}>
                       {index % 5 === 0 ? (
                          // Featured Article Style
                          <div className="border-b border-black pb-4">
                             <div className="bg-gray-200 aspect-video mb-3 flex items-center justify-center border border-gray-300">
                                <Icon size={48} strokeWidth={1} />
                             </div>
                             <h2 className="text-xl font-bold font-serif leading-tight mb-2 group-hover:text-red-700 transition-colors">
                                The Rise of {name}
                             </h2>
                             <p className="text-sm text-gray-600 font-serif leading-relaxed line-clamp-3">
                                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam.
                             </p>
                          </div>
                       ) : (
                          // Standard Entry
                          <div className="flex gap-4 items-start border-b border-gray-200 pb-4">
                             <div className="bg-gray-100 p-2 border border-gray-200 shrink-0">
                                <Icon size={24} />
                             </div>
                             <div>
                                <h3 className="font-bold text-sm font-serif group-hover:underline">{name}</h3>
                                <p className="text-xs text-gray-500 font-sans uppercase mt-1">Vector • {Math.floor(Math.random() * 500)}kb</p>
                             </div>
                          </div>
                       )}
                    </div>
                 ))}
              </div>
           </main>
        </div>
      </div>
    </div>
  );
};

export default Design_Daily;
