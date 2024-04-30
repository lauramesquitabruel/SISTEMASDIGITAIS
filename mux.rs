library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

//multiplexador 4 para 1
entity mux is
    port (
        //I é uma entrada de 4bits
        I : in std_logic_vector (3 downto 0); -- Entrada
        //SEL é uma entrada de seleção de 2bits
        SEL : in std_logic_vector (1 downto 0); -- Seleção
        //Y é a saída
        Y : out std_logic -- Saída
    );
end entity mux;

architecture Comportamento of mux is
begin
    //processo sensível à mudança de SEL e I
    process(SEL, I)
    begin
        case SEL is
            //se o SEL for "00", a saída Y será o bit de índice 0 de I
            when "00" =>
                Y <= I(0);
            when "01" =>
            //saída será o bit de indíce 1
                Y <= I(1);
            when "10" =>
                Y <= I(2);
            when "11" =>
                Y <= I(3);
            when others =>
            //a saída padrão de Y é 0
                Y = "0";   
        end case;
    end process;
end architecture Comportamento;
//refere-se à posição dos bits dentro do vetor I. Se I for "1010", o bit 0 é 1, e assim por diante