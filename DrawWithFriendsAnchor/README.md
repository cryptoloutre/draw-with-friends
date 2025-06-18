# 🎨 Draw with friends with Anchor ⚓️

Program that allows users to draw on the same canvas written with Anchor.

## Installation
Clone the repo:
```
git clone https://github.com/cryptoloutre/draw-with-friends.git
cd draw-with-friends/DrawWithFriendsAnchor
```

## Build
```
anchor build
```

## Deploy
```
solana program deploy target/deploy/draw_with_friends_anchor.so
```

## Update the program ID
Go to the `src/lib.rs` file and update `ID` with the actual deployed program ID. Then, rebuild and redeploy