import type { NextPage } from "next";
import Head from "next/head";
import { MarmotGameView } from "../views";

const MarmotGame: NextPage = (props) => {
  return (
    <div>
      <Head>
        <title>Marmot Game</title>
        <meta
          name="description"
          content="Marmot Game"
        />
      </Head>
      <MarmotGameView />
    </div>
  );
};

export default MarmotGame;
