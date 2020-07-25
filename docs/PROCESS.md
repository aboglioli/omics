# Subscription

1. User subscribe.
2. Subscription generated for current date in status "PendingPayment".
3. Payment confirmation: Subscription status changed to "Paid".

# Authors payment

1. Once a month: static date.
2. Find all Subscription.Status == "Paid" and sum totals. Generate FinancialSummary with total = sum(Suscription.Amount).
3. Find all Contract.Status == "Approved".
4. Generate Summary for all Contracts with status Open.
5. Check if Summary can be paid for Publication according to unique views (business rule).
6. Calculate payment percentage for generated Summaries from FinancialSummary.
7. Pay for each Summary.
