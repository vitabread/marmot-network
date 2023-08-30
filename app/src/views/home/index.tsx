// Next, React
import { FC, useEffect, useState } from 'react';
import Link from 'next/link';
import Image from 'next/image';

// Wallet
import { useWallet, useConnection } from '@solana/wallet-adapter-react';

// Components
import pkg from '../../../package.json';

export const HomeView: FC = ({ }) => {
  const wallet = useWallet();
  const { connection } = useConnection();

  useEffect(() => {
    if (wallet.publicKey) {
      console.log(wallet.publicKey.toBase58())
    }
  }, [wallet.publicKey, connection])

  return (

    <div className="md:hero mx-auto p-4">
      <div className="md:hero-content flex flex-col">
        <div className='mt-6'>
        <div className='text-sm font-normal align-bottom text-right text-slate-600 mt-4'>v{pkg.version}</div>
        <h1 className="text-center text-5xl md:pl-12 font-bold text-transparent bg-clip-text bg-gradient-to-br from-indigo-500 to-fuchsia-500 mb-4">
          Marmot Network (土拨鼠网络)
        </h1>
        </div>
        <div className="flex flex-col mt-2">
          <Image
              src="/marmot_home.gif"
              alt="marmot shoutout"
              width={200}
              height={200}
          />
          <h4 className="md:w-full text-2xl text-slate-300 my-2">
          </h4>
        </div>
        <h4 className="md:w-full text-2x1 md:text-4xl text-center text-slate-300 my-2">
          <p>猜猜土拨鼠在洞外，还是洞内？</p>
          <p className='text-slate-500 text-2x1 leading-relaxed'>猜对获得1枚新铸造的$MARMOT，猜错烧毁1枚已获得的$MARMOT</p>
        </h4>
        <div className="relative group">
          <div className="absolute -inset-0.5 bg-gradient-to-r from-indigo-500 to-indigo-500 rounded-lg blur opacity-40 animate-tilt"></div>
        </div>
      </div>
    </div>
  );
};
