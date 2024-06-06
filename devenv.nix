{ pkgs, ... }: {
	languages.rust = {
		enable = true;
		channel = "stable";
	};

	packages = with pkgs; [ openssl pkg-config sqlx-cli ];

	services.postgres = {
		enable = true;
		package = pkgs.postgresql_16;
		initialDatabases = [{ name = "axolotl_client-api"; }];
		listen_addresses = "127.0.0.1";
	};
}
