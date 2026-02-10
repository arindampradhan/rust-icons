import { useState } from 'react';
import { icons } from '../data';
import { Link } from 'react-router-dom';
import { Search, Coffee, ArrowRight } from 'lucide-react';

const CollectionHome = () => {
  const [searchTerm, setSearchTerm] = useState('');
  
  // Categorized icons mock - In a real app, these would come from your data source
  const categories = {
    'Recent Additions': icons.slice(0, 5),
    'Material Design': icons.slice(5, 10),
    'User Interface': icons.slice(10, 15),
    'Brand Logos': icons.slice(15, 20),
    'System Icons': icons.slice(20, 25),
  };

  return (
    <div className="min-h-screen bg-[#f1f0e8] text-[#1a1a1a] font-serif overflow-x-hidden p-2 md:p-8">
      <div className="max-w-7xl mx-auto bg-[#fbfbf8] shadow-2xl min-h-screen border-x border-black/10 relative">
        
        {/* Header Section */}
        <header className="border-b-4 border-black p-8 text-center bg-white sticky top-0 z-20">
          <div className="flex justify-between items-center border-b border-black pb-2 mb-4 text-xs font-sans font-bold uppercase tracking-widest">
            <span>Vol. CCLVI No. 104</span>
            <span className="flex items-center gap-2">Since 2026 <Coffee size={12}/></span>
            <span>$4.00</span>
          </div>
          
          <Link to="/" className="block group">
            <h1 className="text-6xl md:text-8xl font-black font-serif tracking-tight mb-2 group-hover:opacity-80 transition-opacity">
              The Daily Icon
            </h1>
          </Link>
          
          <div className="italic text-lg text-gray-600 mb-6 font-serif">
            "All the Vectors That Are Fit to Print"
          </div>

          <div className="relative max-w-2xl mx-auto mb-8">
             <div className="inline-flex items-center border-b-2 border-black w-full pb-2 gap-4">
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

          <div className="border-y-2 border-black py-2 flex justify-center gap-8 text-sm font-sans font-bold uppercase overflow-x-auto">
             {Object.keys(categories).map(cat => (
                <a href={`#${cat}`} key={cat} className="hover:underline cursor-pointer whitespace-nowrap scroll-smooth">
                   {cat}
                </a>
             ))}
          </div>
        </header>

        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 p-8">
           {/* Sidebar / Left Column */}
           <aside className="lg:col-span-3 border-r border-black/20 pr-8 hidden lg:block sticky top-64 self-start">
              <h3 className="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">Weather</h3>
              <div className="mb-8 text-sm font-serif">
                 <p>Sunny with a chance of vectors.</p>
                 <p className="font-bold">High 72°F</p>
              </div>

              <h3 className="font-sans font-bold text-xs uppercase tracking-widest border-b border-black mb-4 pb-1">Index</h3>
              <ul className="text-sm font-serif space-y-2">
                 <li className="flex justify-between"><span>Recent</span> <span>A1</span></li>
                 <li className="flex justify-between"><span>Material</span> <span>B2</span></li>
                 <li className="flex justify-between"><span>Brands</span> <span>C4</span></li>
                 <li className="flex justify-between"><span>System</span> <span>D8</span></li>
              </ul>
              
              <div className="mt-12 bg-black text-white p-4 text-center">
                 <h4 className="font-sans font-bold uppercase text-xs mb-2">Advertisement</h4>
                 <div className="border border-white/30 p-4 font-serif italic text-sm">
                    "Got SVG?"
                 </div>
              </div>
           </aside>

           {/* Main Content */}
           <main className="lg:col-span-9 space-y-16">
              {Object.entries(categories).map(([category, items], sectionIndex) => (
                 <section key={category} id={category} className="scroll-mt-64">
                    <div className="flex items-baseline justify-between gap-4 mb-6 border-b-4 border-black pb-2">
                       <h2 className="text-3xl font-black font-serif tracking-tight uppercase">{category}</h2>
                       <Link to={`/collection/all`} className="text-xs font-sans font-bold uppercase tracking-widest hover:underline flex items-center gap-1">
                          See All <ArrowRight size={12} />
                       </Link>
                    </div>
                    
                    {/* Newspaper Style Grid: First item is featured, others are standard */}
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                       {/* Featured Article (First Item) */}
                       <div className="md:col-span-2 lg:col-span-2">
                          {items.slice(0, 1).map((icon) => (
                             <Link key={icon.name} to={`/collection/${icon.name}`} className="group block">
                                <div className="border-b border-black pb-4 h-full">
                                   <div className="bg-gray-100 aspect-[2/1] mb-4 flex items-center justify-center border border-gray-200 relative overflow-hidden">
                                      <div className="absolute inset-0 opacity-5" style={{ backgroundImage: 'radial-gradient(#000 1px, transparent 1px)', backgroundSize: '10px 10px' }}></div>
                                      <icon.Icon size={64} strokeWidth={1} />
                                   </div>
                                   <div className="flex justify-between items-start">
                                      <div>
                                         <h3 className="text-2xl font-bold font-serif leading-tight mb-2 group-hover:text-[#b91c1c] transition-colors">
                                            The Definitive Guide to {icon.name}
                                         </h3>
                                         <p className="text-sm text-gray-600 font-serif leading-relaxed line-clamp-2 max-w-md">
                                            A comprehensive look at the vector construction, usage guidelines, and history of this essential interface element.
                                         </p>
                                      </div>
                                      <div className="text-[10px] font-sans bg-black text-white px-2 py-1 uppercase tracking-widest">
                                         Featured
                                      </div>
                                   </div>
                                </div>
                             </Link>
                          ))}
                       </div>

                       {/* Side Column (Next 2 items stacked) */}
                       <div className="flex flex-col gap-8 border-l border-gray-200 pl-8">
                          {items.slice(1, 3).map((icon) => (
                             <Link to={`/collection/${icon.name}`} key={icon.name} className="group block flex-1">
                                <div className="border-b border-gray-200 pb-4 h-full">
                                   <div className="flex justify-between items-start mb-2">
                                      <h4 className="font-bold font-serif group-hover:text-[#b91c1c] transition-colors">{icon.name}</h4>
                                      <icon.Icon size={20} />
                                   </div>
                                   <p className="text-xs text-gray-500 font-serif line-clamp-3">
                                      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor.
                                   </p>
                                </div>
                             </Link>
                          ))}
                       </div>

                       {/* Bottom Row (Remaining items) */}
                       <div className="col-span-full grid grid-cols-2 md:grid-cols-4 gap-4 border-t border-black pt-4 mt-4">
                          {items.slice(3).map((icon) => (
                             <Link to={`/collection/${icon.name}`} key={icon.name} className="group flex items-center gap-3 hover:bg-gray-50 p-2 transition-colors">
                                <div className="p-2 bg-white border border-gray-200">
                                   <icon.Icon size={16} />
                                </div>
                                <div>
                                   <div className="font-bold font-serif text-sm group-hover:underline">{icon.name}</div>
                                   <div className="text-[10px] text-gray-400 font-sans uppercase">SVG • 24px</div>
                                </div>
                             </Link>
                          ))}
                       </div>
                    </div>
                 </section>
              ))}
           </main>
        </div>
      </div>
    </div>
  );
};

export default CollectionHome;
