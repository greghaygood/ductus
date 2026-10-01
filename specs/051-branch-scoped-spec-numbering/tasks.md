# 051 — Branch-scoped spec numbering Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Spent task sections are pruned per §tasks-phase; git history holds them, and the plan's Technical Decisions hold the decisions they settled.

## 28. Prune plan.md §Implementation notes into the design record

- [x] Move the durable decisions under §Implementation notes into §Technical Decisions or §Trade-offs, and drop the implementation journal, which git history holds
- [x] Re-run /ductus:analyze: the plan-record finding no longer fires

- **Done when**: plan.md has no section outside the design record, every decision the section carried survives in the design record, and analyze no longer reports plan-record for 051
