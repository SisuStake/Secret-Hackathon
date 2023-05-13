# Design

## Overview

<dl>
  <dt>Name</dt>
  <dd>Secret Angels: The Secret Stablecoin</dd>
  <dt>Track</dt>
  <dd>Shade Protocol</dd>
</dl>

### Product: 

An NFT that can wrap LP tokens from the [Shade Pools]([url](https://app.shadeprotocol.io/swap/pools)) powering the [Shade DEX]([url](https://app.shadeprotocol.io/swap)) and then be used for [Shade Borrow]([url](https://app.shadeprotocol.io/borrow)). The value of the NFT needs to be updated based on the performance/value of the LP tokens wrapped into it (or held by it). For the Hackathon it would be ideal to make ONLY the stablecoin pools possible to wrap into the NFT: 

1. Silk + USDC.axl 
2. Silk + USDT.axl 
3. Silk + CMST
4. (BONUS: LP from [Blizzard Finance]([url](https://app.blizzard.finance/pools))): saUSDC + saUSDT

### Product Functions: 

1. Mint an NFT (AI generated) for XXX amount of SCRT tokens

2. Wrap LP Token into NFT 

3. Have a price of NFT based on the value of all wrapped LP Tokens held by the NFT

4. Use NFT to be able to view metrics about the performance of wrapped LPs such as: 

    - Impermanent Loss (See impermanent loss calculator here)

    - Profit/Loss

    - Projected APR (Day %, Week %, Monthly %, Quarterly %, Annual %)

    - Claimable tokens

    - Be able to use NFT with wrapped assets to borrow SILK using Shade Borrow with an LTV of 90%

5. Be able to borrow SILK using the NFT up to a 90% LTV

6. Bonus Features: 

    - Be able to claim all claimable tokens of each LP wrapped into the NFT

    - Be able to auto-compound all LP tokens wrapped into the NFT

    - Be able to sell all LP positions at once 

    - Be able to enter all LP positions held by the NFT at once at even OR pre-determined ratios 

    - Able to buy into a portion of an NFT with a good strategy 

## Screen Shots of Shade Protocol 
### Shade Borrow 
![image](https://user-images.githubusercontent.com/106693799/236632947-0b736b7f-2934-4aaa-a5b5-258c7627603a.png)


### Shade Swap Pools 
![image](https://user-images.githubusercontent.com/106693799/236632953-072f3fbc-51c8-48c1-acc2-dca2f1c7501b.png)


### Metrics Dashboard
![image](https://user-images.githubusercontent.com/106693799/236632959-e1ba90e2-9501-489e-a216-0c63cdad1810.png)


## Figma

I'm using [Figma](https://www.figma.com/) for design.

Please see the Project's [Figma File](https://www.figma.com/file/JLjygiD2g17yfioxFcI9nG/Secret-Hackathon-Dashboard?type=design&t=vBT8HkZPxr3mCsHA-0).

(Please excuse the mess. It is a Hackathon after all!)

### Quick Links
  - [Wireframes](https://www.figma.com/file/JLjygiD2g17yfioxFcI9nG/Secret-Hackathon-Dashboard?type=design&node-id=1-11&t=5pJvnqZ547zBVPZB-0)
