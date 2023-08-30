import { FC } from "react";
import Image from 'next/image';
import { MarmotGame_1 } from '../../components/MarmotGame-1';
import { MarmotGame_2 } from '../../components/MarmotGame-2';

export const MarmotGameView: FC = ({ }) => {

  return (
    <div className="md:hero mx-auto p-4">
      <div className="md:hero-content flex flex-col">
        <h1 className="text-center text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-br from-indigo-500 to-fuchsia-500 mt-10 mb-8">
          猜猜土拨鼠在洞外还是在洞内？
        </h1>
        <div className="relative group">
          <div className="absolute -inset-0.5 bg-gradient-to-r from-indigo-500 to-indigo-500 rounded-lg blur opacity-40 animate-tilt"></div>
          <div className="max-w-md mx-auto mockup-code bg-primary border-2 border-[#5252529f] p-6 px-10 my-2">
              <Image
                  src="/marmot_game.png"
                  alt="marmot game"
                  width={1024}
                  height={512}
              />
          </div>

        </div>
        <div className='text-center flex flex-row ml-1'>
        </div>

        {/* CONTENT GOES HERE */}
        <div className="text-center">
          <MarmotGame_1 />
          <MarmotGame_2 />
        </div>
      </div>
    </div>
  );
};
