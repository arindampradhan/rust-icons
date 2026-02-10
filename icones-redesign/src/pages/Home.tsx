import { Link } from 'react-router-dom';
import { Newspaper, Circle, Box, Grid3X3, Ruler, TrendingUp, Mic, Radio, Dna, Box as Box3D } from 'lucide-react';

const Home = () => {
  return (
    <div className="min-h-screen bg-neutral-950 text-white flex flex-col items-center justify-center font-sans p-8">
      <h1 className="text-5xl font-bold mb-4 tracking-tight">Icones Redesign</h1>
      
      {/* Top 5 Section */}
      <h2 className="text-2xl font-bold text-yellow-500 mb-6 mt-4 uppercase tracking-widest border-b border-yellow-500/30 pb-2">The Top 5</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4 max-w-7xl w-full mb-16">
        
        {/* Rank 1 */}
        <Link to="/1" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-yellow-500/50 transition-all hover:scale-[1.02] relative overflow-hidden">
          <div className="absolute top-0 right-0 bg-yellow-500 text-black text-xs font-bold px-2 py-1">#1</div>
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <Newspaper size={32} className="text-gray-200" />
             <h2 className="text-lg font-semibold leading-tight">The Daily Icon</h2>
          </div>
        </Link>

        {/* Rank 2 */}
        <Link to="/2" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-neutral-500 transition-all hover:scale-[1.02] relative overflow-hidden">
          <div className="absolute top-0 right-0 bg-neutral-700 text-white text-xs font-bold px-2 py-1">#2</div>
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <Circle size={32} className="text-neutral-100" />
             <h2 className="text-lg font-semibold leading-tight">Zen Void</h2>
          </div>
        </Link>

        {/* Rank 3 */}
        <Link to="/3" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-blue-500 transition-all hover:scale-[1.02] relative overflow-hidden">
          <div className="absolute top-0 right-0 bg-neutral-800 text-neutral-400 text-xs font-bold px-2 py-1">#3</div>
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <Box size={32} className="text-blue-500" />
             <h2 className="text-lg font-semibold leading-tight">The Archivist</h2>
          </div>
        </Link>

        {/* Rank 4 */}
        <Link to="/4" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-orange-500 transition-all hover:scale-[1.02] relative overflow-hidden">
           <div className="absolute top-0 right-0 bg-neutral-800 text-neutral-400 text-xs font-bold px-2 py-1">#4</div>
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <Grid3X3 size={32} className="text-orange-500" />
             <h2 className="text-lg font-semibold leading-tight">Swiss Int'l</h2>
          </div>
        </Link>

        {/* Rank 5 */}
        <Link to="/5" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-blue-400 transition-all hover:scale-[1.02] relative overflow-hidden">
           <div className="absolute top-0 right-0 bg-neutral-800 text-neutral-400 text-xs font-bold px-2 py-1">#5</div>
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <Ruler size={32} className="text-blue-400" />
             <h2 className="text-lg font-semibold leading-tight">Blueprint</h2>
          </div>
        </Link>
      </div>

      {/* Challengers Section */}
      <h2 className="text-2xl font-bold text-neutral-400 mb-6 uppercase tracking-widest border-b border-neutral-800 pb-2">Round 2 Challengers</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4 max-w-7xl w-full">
        
        <Link to="/6" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-green-500 transition-all hover:scale-[1.02]">
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <TrendingUp size={32} className="text-green-500" />
             <h2 className="text-lg font-semibold leading-tight text-neutral-300">Terminal</h2>
             <p className="text-xs text-neutral-600">Financial Data</p>
          </div>
        </Link>

        <Link to="/7" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-orange-600 transition-all hover:scale-[1.02]">
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <Mic size={32} className="text-orange-600" />
             <h2 className="text-lg font-semibold leading-tight text-neutral-300">Synthesizer</h2>
             <p className="text-xs text-neutral-600">Modular Audio</p>
          </div>
        </Link>

        <Link to="/8" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-lime-400 transition-all hover:scale-[1.02]">
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <Radio size={32} className="text-lime-400" />
             <h2 className="text-lg font-semibold leading-tight text-neutral-300">Broadcast</h2>
             <p className="text-xs text-neutral-600">Analog CRT</p>
          </div>
        </Link>

        <Link to="/9" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-cyan-400 transition-all hover:scale-[1.02]">
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <Dna size={32} className="text-cyan-400" />
             <h2 className="text-lg font-semibold leading-tight text-neutral-300">Biolab</h2>
             <p className="text-xs text-neutral-600">Organic Interface</p>
          </div>
        </Link>

        <Link to="/10" className="group p-6 rounded-2xl bg-neutral-900 border border-neutral-800 hover:border-white transition-all hover:scale-[1.02]">
          <div className="flex flex-col items-center gap-4 mb-2 text-center">
             <Box3D size={32} className="text-white" />
             <h2 className="text-lg font-semibold leading-tight text-neutral-300">Z-Axis</h2>
             <p className="text-xs text-neutral-600">3D Space</p>
          </div>
        </Link>

      </div>
    </div>
  );
};

export default Home;
