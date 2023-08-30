import { useAnchorWallet, useConnection, useWallet } from '@solana/wallet-adapter-react';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction, TransactionMessage, TransactionSignature, VersionedTransaction, u64 } from '@solana/web3.js';
import { FC, useCallback } from 'react';
import { notify } from "../utils/notifications";
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import { MarmotNetwork, IDL} from "../idl/marmot_network";

import { Metaplex } from "@metaplex-foundation/js";
import { getMint, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID, MintLayout } from "@solana/spl-token";

export const MarmotGame_1: FC = () => {
    const { connection } = useConnection();
    const { publicKey, sendTransaction } = useWallet();

    const anchorWallet = useAnchorWallet();
    const anchorProvider = new AnchorProvider(connection, anchorWallet, {});

    const programId = new PublicKey("JkTuLCESE1jLwBrGHBdbv1qx8oTsD5FHw4EgMLvbHam");
    const program = new Program<MarmotNetwork>(IDL, programId, anchorProvider);


    // metaplex token metadata program ID
    const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );

    // metaplex setup
    const metaplex = Metaplex.make(connection);

    // token metadata
    const metadata = {
      uri: "https://raw.githubusercontent.com/vitabread/marmot-network-file/main/spl-token.json",
      name: "Marmot Coin",
      symbol: "MARMOT",
    };

    // reward token mint PDA
    const [rewardTokenMintPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("reward")],
      programId
    );

    console.log("rewardTokenMintPDA: " + rewardTokenMintPDA);



    const onClick = useCallback(async () => {
        if (!publicKey) {
            notify({ type: 'error', message: `Wallet not connected!` });
            console.log('error', `Send Transaction: Wallet not connected!`);
            return;
        }

        /*
        // Get Mint PDA
        const mintData = await getMint(connection, rewardTokenMintPDA);
        console.log("Mint Already Exists");
        console.log(rewardTokenMintPDA.toString());
        */

        let signature: TransactionSignature = '';
        try {

            const instructions: TransactionInstruction[] = [];

            // reward token mint metadata account address
            const rewardTokenMintMetadataPDA = await metaplex
              .nfts()
              .pdas()
              .metadata({ mint: rewardTokenMintPDA });

              // player data account PDA
            const [playerPDA] = await PublicKey.findProgramAddressSync(
                [Buffer.from("player"), publicKey.toBuffer()],
                programId
              );
            console.log("playerPDA: " + playerPDA.toString());

              // player token account address
            const playerTokenAccount = await getAssociatedTokenAddressSync(
                rewardTokenMintPDA,
                publicKey
              );
            console.log("playerTokenAccount: " + playerTokenAccount.toString());

            /*
            const txHash1 = await program.methods
                .initPlayer()
                .accounts({
                  playerData: playerPDA,
                  player: publicKey,
                })
                .instruction();
            instructions.push(txHash1);
            */

            const txHash = await program.methods
                .guessOne()
                .accounts({
                  player: publicKey,
                  playerData: playerPDA,
                  playerTokenAccount: playerTokenAccount,
                  rewardTokenMint: rewardTokenMintPDA,
                })
                .instruction();
            instructions.push(txHash);

            // Get the lates block hash to use on our transaction and confirmation
            let latestBlockhash = await connection.getLatestBlockhash();

            // Create a new TransactionMessage with version and compile it to legacy
            const messageLegacy = new TransactionMessage({
                payerKey: publicKey,
                recentBlockhash: latestBlockhash.blockhash,
                instructions,
            }).compileToLegacyMessage();

            // Create a new VersionedTransacction which supports legacy and v0
            const transation = new VersionedTransaction(messageLegacy);

            // Send transaction and await for signature
            signature = await sendTransaction(transation, connection);

            // Send transaction and await for signature
            await connection.confirmTransaction({ signature, ...latestBlockhash }, 'confirmed');

            console.log(signature);

            const playerData = await program.account.playerData.fetch(playerPDA);
            console.log("playData Win: " + playerData.win);
            console.log("playData Lose: " + playerData.lose);

            notify({ type: 'success', message: '交易成功! ' + '亲，您的最新成绩：' + "(获胜次数: " + playerData.win + ", 失败次数: " + playerData.lose + ")", txid: signature });
        } catch (error: any) {
            notify({ type: 'error', message: `交易失败!`, description: error?.message, txid: signature });
            console.log('error', `交易失败! ${error?.message}`, signature);
            return;
        }
    }, [publicKey, notify, connection, sendTransaction]);

    return (
        <div className="flex flex-row justify-center">
            <div className="relative group items-center">
                <div className="m-1 absolute -inset-0.5 bg-gradient-to-r from-indigo-500 to-fuchsia-500
                rounded-lg blur opacity-20 group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-tilt"></div>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={onClick} disabled={!publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            在洞外
                        </span>
                    </button>
             </div>
        </div>
    );
};
