library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity contador is
    port(
        EN: in std_logic;
        CLOCK: in std_logic;
        RESET: in std_logic;
        //vai um
        BIT_OVERFLOW: out std_logic;
        //valor da contagem de 4bits
        COUNT: out std_logic_vector(3 downto 0);
    );
end contador;

architecture Comportamento of contador is
//sinal interno counter do tipo inteiro com um intervalo de 0 a 9
//é o contador decimal
    signal counter: integer range 0 to 9: = 0;
begin
    process(CLOCK, RESET)
    begin
        //se o reset está ativado
        if RESET = '1' then
            //reseta o contador e o bit de overflow p 0
            counter <= 0;
            BIT_OVERFLOW <= 0;
        //se não tiver reset
        //&& ocorrer uma borda de subida do clock e o enable estiver ativado
        elseif rising_edge(CLOCK) and EN = '1' then
            //se o contador atingiu o valor máximo (9)
            if counter = 9 then
                //contador vai p 0 e o bit de overflow p 1
                counter <= 0;
                BIT_OVERFLOW <= '1';
            else
            //se não, incrementa 1 no contador e o bit de overflow é 0
                counter <= counter + 1;
                BIT_OVERFLOW <= '0';
            end if;
        end if;
    end process;

    //converte o valor do contador para std_logic_vector para a saída COUNT.
    COUNT <= std_logic_vector(to_unsignded(counter, COUNT'lenght));
end architecture Comportamento;