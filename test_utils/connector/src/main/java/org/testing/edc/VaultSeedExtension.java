package org.testing.edc;

import org.eclipse.edc.runtime.metamodel.annotation.Inject;
import org.eclipse.edc.spi.security.Vault;
import org.eclipse.edc.spi.system.ServiceExtension;
import org.eclipse.edc.spi.system.ServiceExtensionContext;

import java.util.Map;

public class VaultSeedExtension implements ServiceExtension {


    public static final String VAULT_TESTING_PREFIX = "testing.edc.vaults";

    public static final String VAULT_TESTING_KEY = "key";
    public static final String VAULT_TESTING_VALUE = "value";

    @Inject
    private Vault vault;


    @Override
    public void initialize(ServiceExtensionContext context) {
        ServiceExtension.super.initialize(context);

        var config = context.getConfig(VAULT_TESTING_PREFIX);
        var secrets = config.partition().map((partition) -> {
            var key = partition.getString(VAULT_TESTING_KEY);
            var value = partition.getString(VAULT_TESTING_VALUE);
            return Map.entry(key,value);
        }).toList();

        secrets.forEach(secret -> vault.storeSecret(secret.getKey(), secret.getValue()));
    }
}
