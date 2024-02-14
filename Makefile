build:
	archway contracts build
store:
	archway contracts store nameservice
init:
	archway contracts instantiate nameservice --args '{"purchase_price": {"amount": "10", "denom": "aconst"}}'
register:
	archway contracts execute nameservice --args '{"register": {"name": "chris"}}' --amount 11aconst
query:
	archway contracts query smart nameservice --args '{"record": {"name": "chris"}}'
transfer:
	archway contracts execute nameservice --args '{"transfer": {"name": "chris", "to": "archway1vfmx5el9xzp9cgk4qlvj33j3vcqeywhzcdhfl3"}}' --amount 12aconst
.PHONY: build store init register
