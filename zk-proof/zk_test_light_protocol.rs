// Define recipient solana address
const recipient = 'BTknD32D76aoNGfMce2RsZTSBFE1SqLkpdZgdZ3RfmCW';
// Define amount in lamports
const amount = '1000000'

// Trigger popup widget
const popupWindow = window.open(
    `https://widget.lightprotocol.com?token=${'SOL'}&recipient=${recipient}&amount=${amount}`,
    '_blank', // https://dev.widget.lightprotocol.com for devnet
    `left=0,top=0,width=500,height=700`,
); 
