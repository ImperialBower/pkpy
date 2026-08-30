---
type: API Surface
title: Table, session and hand lifecycle
description: Dealer, PokerSession, the NoCell table types, chip settlement, and Pluribus hand logs.
tags: [api, table, session, lifecycle]
timestamp: 2026-08-30T00:00:00Z
---

# Two ways to drive a hand

| Approach | Classes | Character |
|---|---|---|
| `Dealer` | `Dealer`, `TableAction`, `TableLog` | Lower level. Explicit `DealerAction` steps, `advance_street`, `end_hand`. |
| `PokerSession` | `PokerSession`, `PlayerAction`, `SessionStep` | Guided. The session drives the lifecycle and reports the next required step. |

`PokerSession` is the one to reach for when you want the library to enforce hand
order. `Dealer` is the one to reach for when you want to control it yourself.

# Table state

`TableNoCell`, `PlayerNoCell`, `SeatNoCell`, `SeatsNoCell` are the cell-free
table types (`src/table_no_cell.rs`), covered by `tests/test_table_no_cell.py`.
`Player`, `Stack`, `PlayerState`, `ForcedBets` and `Seatbit` fill in the seat and
chip detail.

# Settlement

| Class | Role |
|---|---|
| `Winnings` | The full payout result of a hand. Returned by `end_hand()`. |
| `PotWin` | One pot's award. |
| `SeatEquity` | Chips won by a set of seats. |

# Hand logs

`Pluribus` and `PluribusEvent` read the Pluribus log format.

> Upstream `pkcore` 0.10.0 added the **write** half of that format (the
> `Unumable` trait and `Pluribus::write_log`). It is **not yet wrapped here.**
> See [upstream dependency](/decisions/pkcore-upgrades.md).

# Not yet wrapped

`pkcore` 0.11.0 added table snapshot/restore (`Table::snapshot`,
`PokerSession::snapshot`) and the finer `Table::showdown` /
`Table::audit_chip_total` tier beneath `end_hand`. Neither is exposed to Python
yet. These are the most obvious next additions to this area.

# Related

- [Module map](/architecture/module-map.md)
- [Upstream dependency](/decisions/pkcore-upgrades.md)
