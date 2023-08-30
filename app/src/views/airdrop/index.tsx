import { FC } from "react";
import Image from 'next/image';
import { AirDropToken } from '../../components/AirDropToken';

export const AirDropView: FC = ({ }) => {

  return (
    <div className="md:hero mx-auto p-4">
      <div className="md:hero-content flex flex-col">
        <h1 className="text-center text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-br from-indigo-500 to-fuchsia-500 mt-10 mb-8">
          获得 1枚 $MARMOT
        </h1>
        <div className='text-center flex flex-row ml-1'>
          <Image
              src="/marmot_coin.png"
              alt="marmot coin"
              width={80}
              height={80}
          />
        </div>

        {/* CONTENT GOES HERE */}
        <div className="text-center">
          <AirDropToken />
        </div>
      </div>
    </div>
  );
};
