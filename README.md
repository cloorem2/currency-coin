# Currency Coin
Currency for a new era

Four coins

Currency coin: CC,

Two bonds: CCB0 and CCB1,

One short bond: CCS0.


CC is the underlying of the bond and the short, so interest is paid
and measured in CC. Interest is paid into the inbuilt AMMs which may be
extracted at any time by selling.

Once a week the bonds mature and should be redeemed for the other bond,
of which only one is active at a time. Redemption pays the interest in two
parts, half in more bonds and the other half in CC. This process alows the
protocol to accomplish several things.

The bonds are repriced back to a CC.

CC is put into circulation.

And the bonds increase their liquidity in the pool.

More subtly this means that you don't have to sell to extract interest,
so that forces on the interest rate are balanced.


# The interest rate is affected only by buying and selling
Buying the short increases the rate by the same amount as selling the bond,
and vice versa.
branch test
