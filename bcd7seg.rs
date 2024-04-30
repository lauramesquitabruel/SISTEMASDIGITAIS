library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity bcd7seg is
//converte-se um número bcd(decimal codificado em binário) num display de 7 segmentos
    port (
        //enable habilita ou desabilita o funcionamento do circuito
        EN : in std_logic; -- Habilitação
        //D é uma entrada de 4bits que representa o número bcd a ser codificado
        D : in std_logic_vector (3 downto 0); -- Entrada do Codificador BCD
        //S é uma saída de 7bits 
        //que ativa os segmentos correspondentes no display de 7 segmentos 
        //para representar o número bcd da entrada
        S : out std_logic_vector (6 downto 0) -- Saída do Codificador BCD
    );
end entity bcd7seg;

architecture Comportamento of bcd7seg is
begin
    //processo sensível à mudança de valor de EN e D
    process(EN, D)
    begin
        //se o enable está ativo
        if EN = '1' then
            case D is
                when "0000" =>
                    S <= "0000001";
                when "0001" =>
                    S <= "1001111";
                when "0010" =>
                    S <= "0010010";
                when "0011" =>
                    S <= "0000110";
                when "0100" =>
                    S <= "1001100";
                when "0101" =>
                    S <= "0100100";
                when "0111" =>
                    S <= "0001111";
                when "1000" =>
                    S <= "0000000";
                when "1001" =>
                    S <= "0000100";
                when other =>
                    S <= "1111111";
            end case;
        else
        //se o enable estiver desativado, todas as saídas são "1111111"
            S <= "1111111";
        end if;
    end process;
end architecture Comportamento;