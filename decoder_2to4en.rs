library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity decoder_2to4en is
    port (
        //EN é a entrada de habilitação (enable)
        EN : in std_logic; -- Habilitação
        //A é a entrada do decodificador com 2bits
        A : in std_logic_vector (1 downto 0); -- Entrada do Decodificador
        // Y_L é a saída do decodificador com 4bits
        Y_L : out std_logic_vector (3 downto 0) -- Saída do Decodificador (ativo baixo)
    );
end entity decoder_2to4en;

architecture Comportamento of decoder_2to4en is
begin
    //na maioria dos casos não se coloca variáveis de saída para serem analisadas no processo
    process(EN, A)
    begin
        //quando o enable está ativo
        if EN = "1" then
            case A is
                //se A recebe "00"
                when "00" =>
                    //a saída Y_L é "1110"
                    Y_L <= "1110";
                when "01" =>
                    Y_L <= "1101";
                when "10" =>
                    Y_L <= "1011";
                when "11" =>
                    Y_L <= "0111";
                when others =>
                    Y_L <= "1111"
                //julga-se os casos conforme a tabela verdade do decodificador
            end case;
        else 
        //se o enable está desativado, a saída Y_L é "1111";
            Y_L <= "1111";
        end if;
    end process;
end architecture Comportamento;