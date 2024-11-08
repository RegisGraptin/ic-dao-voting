import { ConnectButton } from '@rainbow-me/rainbowkit';
import type { NextPage } from 'next';
import Head from 'next/head';
import styles from '../styles/Home.module.css';
import { useReadContract, useReadContracts, useWriteContract } from 'wagmi';

import DAOContract from "../abi/DAO.json";
import { useState } from 'react';

const Home: NextPage = () => {
  
  const { data, error, writeContract } = useWriteContract()

  const [btcAddress, setBtcAddress] = useState("");
  const [btcValue, setBtcValue] = useState(0);

  const createProposal = () => {
    writeContract({
      abi: DAOContract.abi,
      address: '0xA6E782af1b182329282CC67f1ce0f4680030E12F',
      functionName: 'sendBTCProposal',
      args: [btcAddress, btcValue],
    })  
  }

  const voteInFavor = (proposalId: Number) => {
    writeContract({
      abi: DAOContract.abi,
      address: '0xA6E782af1b182329282CC67f1ce0f4680030E12F',
      functionName: 'voteOnProposal',
      args: [proposalId],
    });
  }

  const { data: lastProposalId, isLoading: lastProposalIdLoading } = useReadContract({
    address: "0xA6E782af1b182329282CC67f1ce0f4680030E12F",
    abi: DAOContract.abi,
    functionName: 'proposalIds',
    args: [],
})

  const { data: proposals, isLoading: proposalsLoading } = useReadContracts({
    contracts: Array.from({ length: Number(lastProposalId) }).map(
    (_, index) => ({
        abi: DAOContract.abi,
        address: "0xA6E782af1b182329282CC67f1ce0f4680030E12F",
        functionName: "proposals",
        args: [index],
    })
    ),
  });

  console.log(lastProposalId);

  return (
    <div className={styles.container}>
      <Head>
        <title>RainbowKit App</title>
        <meta
          content="Generated by @rainbow-me/create-rainbowkit"
          name="description"
        />
        <link href="/favicon.ico" rel="icon" />
      </Head>

      <main className={styles.main}>
        <ConnectButton />

        <h1 className={styles.title}>
          BTC Proposal DAO
        </h1>

        <p className={styles.description}>
          Number of Proposals: {Number(lastProposalId)}
        </p>

        <div className={styles.grid}>

          {proposals && proposals.map(function (proposal, i) {
            console.log(proposal);

            return <>
              <div className={styles.card} key={i}>
                <h2>Proposal #{i}</h2>
                <p>{proposal.result[0]} - {Number(proposal.result[1])}</p>
                
                <button 
                  type="button" 
                  className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2" 
                  onClick={() => voteInFavor(i)}
                >
                  Vote in favor
                </button>

              </div>
            </>
          })}

          
        </div>

        <div
            
          >
            <h2 className='text-4xl font-extrabold'>Create BTC Proposal</h2>

            <div className="mb-6">
              <div>
                <label htmlFor="btcAddress" className="block mb-2 text-sm font-medium text-gray-900">BTC Address</label>
                <input 
                  type="text" 
                  className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
                  placeholder="1A1zP1eP..." 
                  required 
                  value={btcAddress} 
                  onChange={(e) => setBtcAddress(e.target.value)}
                />
            </div>
            </div>
            <div className="mb-6">
            <div>
                <label htmlFor="btcValue" className="block mb-2 text-sm font-medium text-gray-900">BTC Value</label>
                <input 
                  type="text" 
                  className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
                  placeholder="1" 
                  required 
                  value={btcValue} 
                  onChange={(e) => setBtcValue(Number(e.target.value))}
                />
            </div>
            </div>
            
            <div className="mb-6">
              <div>
                {error?.message}
              </div>

            <button 
              type="button" 
              className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2" 
              onClick={createProposal}
            >
                Create proposal
            </button>

            </div>


          </div>
      </main>

      <footer className={styles.footer}>
        <a href="https://rainbow.me" rel="noopener noreferrer" target="_blank">
          Made with ❤️ by your frens at 🌈
        </a>
      </footer>
    </div>
  );
};

export default Home;