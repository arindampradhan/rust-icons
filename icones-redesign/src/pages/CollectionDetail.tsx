import { useState } from 'react';
import { icons } from '../data';
import { Link, useParams } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import { Search, ArrowLeft, Grid, List, Sun, Star, Menu, Download, Copy, Code, ChevronRight } from 'lucide-react';

const CollectionDetail = () => {
  const { id } = useParams();
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedIcon, setSelectedIcon] = useState<any>(null);

  // Group icons by first letter for "Rust Icons" style categories
  const groupedIcons = icons.filter(icon =>
    icon.name.toLowerCase().includes(searchTerm.toLowerCase())
  ).reduce((acc, icon) => {
    const letter = icon.name.charAt(0).toUpperCase();
    if (!acc[letter]) acc[letter] = [];
    acc[letter].push(icon);
    return acc;
  }, {} as Record<string, typeof icons>);

  return (
    <div className="min-h-screen bg-[#f1f0e8] text-[#1a1a1a] font-serif overflow-hidden flex">
       {/* Sidebar Navigation */}
       <aside className="w-64 border-r-2 border-black bg-[#fbfbf8] hidden md:flex flex-col h-screen sticky top-0">
          <div className="p-4 border-b border-black flex items-center gap-2">
             <Link to="/" className="hover:bg-black hover:text-white p-2 rounded-full transition-colors">
                <ArrowLeft size={20} />
             </Link>
             <h1 className="font-serif font-bold text-xl truncate">Material Symbols</h1>
          </div>
          
          <div className="p-4 border-b border-black bg-white">
             <div className="relative">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" size={16} />
                <input 
                   type="text" 
                   placeholder="Search category..." 
                   className="w-full bg-gray-50 border border-gray-200 py-2 pl-9 pr-2 text-sm focus:outline-none focus:border-black font-sans"
                />
             </div>
          </div>

          <div className="flex-1 overflow-y-auto">
             <div className="p-2 space-y-1 font-sans text-sm">
                <div className="px-3 py-2 bg-black text-white font-bold cursor-pointer">All <span className="float-right text-xs opacity-70">15049</span></div>
                <div className="px-3 py-2 hover:bg-gray-200 cursor-pointer">Recent <span className="float-right text-xs text-gray-500">2</span></div>
                <div className="my-2 border-t border-gray-200"></div>
                <div className="px-3 py-2 hover:bg-gray-200 cursor-pointer font-bold">Material Symbols</div>
                <div className="px-3 py-2 hover:bg-gray-200 cursor-pointer">Google Material</div>
                <div className="px-3 py-2 hover:bg-gray-200 cursor-pointer">Material Design</div>
                <div className="px-3 py-2 hover:bg-gray-200 cursor-pointer">Material Light</div>
             </div>
          </div>

          <div className="p-4 border-t border-black text-xs font-sans text-center text-gray-500">
             © 2026 Icônes
          </div>
       </aside>

       {/* Main Content */}
       <main className="flex-1 h-screen overflow-y-auto bg-white relative">
          {/* Header */}
          <header className="sticky top-0 bg-white/95 backdrop-blur border-b border-black z-10 px-6 py-4 flex justify-between items-center">
             <div className="flex flex-col">
                <div className="flex items-center gap-2">
                   <h2 className="text-2xl font-black font-serif">Material Symbols</h2>
                   <a href="#" className="text-gray-400 hover:text-black"><Star size={16} /></a>
                </div>
                <div className="text-xs text-gray-500 font-sans mt-1">@google • Apache 2.0 • 15049 icons</div>
             </div>

             <div className="flex items-center gap-4">
                <div className="relative w-64">
                   <input 
                      type="text" 
                      placeholder="Search..." 
                      value={searchTerm}
                      onChange={(e) => setSearchTerm(e.target.value)}
                      className="w-full border-b-2 border-gray-200 py-1 text-lg font-serif focus:outline-none focus:border-black bg-transparent"
                   />
                </div>
                <div className="flex gap-2 border-l border-gray-200 pl-4">
                   <button className="p-2 hover:bg-gray-100"><Sun size={20}/></button>
                   <button className="p-2 hover:bg-gray-100"><Grid size={20}/></button>
                   <button className="p-2 hover:bg-gray-100"><Menu size={20}/></button>
                </div>
             </div>
          </header>
          
          {/* Variant Chips (New Addition) */}
          <div className="px-6 py-4 border-b border-gray-200 flex gap-2 overflow-x-auto bg-[#fbfbf8]">
             {['Outlined', 'Rounded', 'Sharp', 'Two Tone'].map(variant => (
                <button key={variant} className="px-4 py-1 border border-gray-300 rounded-full text-xs font-sans hover:bg-black hover:text-white hover:border-black transition-colors whitespace-nowrap">
                   {variant}
                </button>
             ))}
          </div>

          {/* Icons Grid with Categories */}
          <div className="p-6 space-y-8">
             {Object.entries(groupedIcons).sort().map(([letter, items]) => (
                <div key={letter} className="break-inside-avoid">
                   <div className="flex items-center gap-4 mb-4 border-b border-black pb-2">
                      <h3 className="text-4xl font-black font-serif text-gray-200">{letter}</h3>
                      <div className="h-[1px] flex-1 bg-gray-200"></div>
                      <span className="text-xs font-sans text-gray-400 font-bold">{items.length} ICONS</span>
                   </div>
                   
                   <div className="grid grid-cols-[repeat(auto-fill,minmax(100px,1fr))] gap-x-4 gap-y-8">
                      {items.map((icon) => (
                         <button 
                            key={icon.name} 
                            onClick={() => setSelectedIcon(icon)}
                            className={`group flex flex-col items-start gap-2 p-2 hover:bg-gray-50 rounded transition-colors ${selectedIcon?.name === icon.name ? 'bg-black text-white hover:bg-black hover:text-white' : ''}`}
                         >
                            <div className={`aspect-square w-full flex items-center justify-center border border-gray-200 bg-white ${selectedIcon?.name === icon.name ? 'border-gray-600 bg-gray-800 text-white' : ''}`}>
                               <icon.Icon size={32} strokeWidth={1} />
                            </div>
                            <div className="w-full text-left">
                               <div className="font-serif font-bold text-sm truncate w-full">{icon.name}</div>
                               <div className="font-sans text-[10px] text-gray-400 truncate">SVG • 24px</div>
                            </div>
                         </button>
                      ))}
                   </div>
                </div>
             ))}
          </div>
       </main>

       {/* Detail Drawer */}
       <AnimatePresence>
          {selectedIcon && (
             <motion.div 
                initial={{ y: "100%" }}
                animate={{ y: 0 }}
                exit={{ y: "100%" }}
                transition={{ type: "spring", damping: 25, stiffness: 200 }}
                className="fixed bottom-0 right-0 left-0 md:left-64 bg-white border-t-4 border-black shadow-[0_-10px_40px_rgba(0,0,0,0.1)] z-50 h-[400px] flex flex-col"
             >
                {/* Drawer Header */}
                <div className="flex justify-between items-center p-4 border-b border-gray-200 bg-[#fbfbf8]">
                   <h3 className="text-2xl font-black font-serif">{selectedIcon.name}</h3>
                   <button onClick={() => setSelectedIcon(null)} className="p-2 hover:bg-gray-200 rounded-full">
                      <ArrowLeft className="rotate-[-90deg]" />
                   </button>
                </div>

                <div className="flex-1 flex overflow-hidden">
                   {/* Large Preview */}
                   <div className="w-1/3 bg-[#f1f0e8] flex items-center justify-center border-r border-gray-200 relative p-8">
                      <selectedIcon.Icon size={180} strokeWidth={1} />
                      <div className="absolute top-4 left-4 font-sans text-xs font-bold bg-white px-2 py-1 border border-black">PREVIEW</div>
                   </div>

                   {/* Controls & Snippets */}
                   <div className="flex-1 p-8 overflow-y-auto">
                      <div className="mb-8">
                         <h4 className="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">Actions</h4>
                         <div className="flex gap-4">
                            <button className="flex items-center gap-2 bg-black text-white px-4 py-2 font-sans font-bold text-sm hover:bg-[#b91c1c] transition-colors">
                               <Copy size={16} /> Copy SVG
                            </button>
                            <button className="flex items-center gap-2 border border-black px-4 py-2 font-sans font-bold text-sm hover:bg-gray-50 transition-colors">
                               <Code size={16} /> Copy JSX
                            </button>
                            <button className="flex items-center gap-2 border border-black px-4 py-2 font-sans font-bold text-sm hover:bg-gray-50 transition-colors">
                               <Download size={16} /> Download
                            </button>
                         </div>
                      </div>

                      <div className="grid grid-cols-2 gap-8">
                         <div>
                            <h4 className="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">Snippets</h4>
                            <div className="space-y-2">
                               <div className="font-mono text-xs bg-gray-50 p-2 border border-gray-200 truncate">
                                  &lt;{selectedIcon.name} /&gt;
                                </div>
                               <div className="font-mono text-xs bg-gray-50 p-2 border border-gray-200 truncate">
                                  import &#123; {selectedIcon.name} &#125; from 'lucide-react';
                               </div>
                            </div>
                         </div>
                         <div>
                            <h4 className="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">Links</h4>
                            <div className="flex gap-2">
                               <button className="px-3 py-1 border border-gray-300 text-xs font-sans hover:border-black">Unpkg</button>
                               <button className="px-3 py-1 border border-gray-300 text-xs font-sans hover:border-black">CDN</button>
                               <button className="px-3 py-1 border border-gray-300 text-xs font-sans hover:border-black">GitHub</button>
                            </div>
                         </div>
                      </div>
                   </div>
                </div>
             </motion.div>
          )}
       </AnimatePresence>
    </div>
  );
};

export default CollectionDetail;
