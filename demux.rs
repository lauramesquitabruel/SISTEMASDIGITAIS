library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity demux is
    port (
        //I é uma entrada de 16 bits
        I: in std_logic_vector(15 downto 0);
        //SEL é uma entrada de seleção de 3bits
        SEL : in std_logic_vector(2 downto 0);
        //oito saídas de 16bits
        YO, Y1, Y2, Y3, Y4, Y5, Y6, Y7: out std_logic_vector(15 downto 0);
    );
end entity demux;

architecture Comportamento of demux is
begin 
    process(SEL, I)
    begin
        case SEL is
            when "000" =>
                Y0 <= I;
                Y1 <= (others => '0');
                Y2 <= (others => '0');
                Y3 <= (others => '0');
                Y4 <= (others => '0');
                Y5 <= (others => '0');
                Y6 <= (others => '0');
                Y7 <= (others => '0');
            when "001" =>
                Y1 <= I;
                Y0 <= (others => '0');
                Y2 <= (others => '0');
                Y3 <= (others => '0');
                Y4 <= (others => '0');
                Y5 <= (others => '0');
                Y6 <= (others => '0');
                Y7 <= (others => '0');
            when "010" =>
            //por exemplo, como "010" é 2, Y2 recebe I e o resto vai p 0
                Y2 <= I;
                Y1 <= (others => '0');
                Y0 <= (others => '0');
                Y3 <= (others => '0');
                Y4 <= (others => '0');
                Y5 <= (others => '0');
                Y6 <= (others => '0');
                Y7 <= (others => '0');
            when "011" =>
                Y3 <= I;
                Y1 <= (others => '0');
                Y2 <= (others => '0');
                Y0 <= (others => '0');
                Y4 <= (others => '0');
                Y5 <= (others => '0');
                Y6 <= (others => '0');
                Y7 <= (others => '0');
            when "100" =>
                Y4 <= I;
                Y1 <= (others => '0');
                Y2 <= (others => '0');
                Y3 <= (others => '0');
                Y0 <= (others => '0');
                Y5 <= (others => '0');
                Y6 <= (others => '0');
                Y7 <= (others => '0');
            when "101" =>
                Y5 <= I;
                Y1 <= (others => '0');
                Y2 <= (others => '0');
                Y3 <= (others => '0');
                Y4 <= (others => '0');
                Y0 <= (others => '0');
                Y6 <= (others => '0');
                Y7 <= (others => '0');
            when "110" =>
                Y6 <= I;
                Y1 <= (others => '0');
                Y2 <= (others => '0');
                Y3 <= (others => '0');
                Y4 <= (others => '0');
                Y5 <= (others => '0');
                Y0 <= (others => '0');
                Y7 <= (others => '0');
            when "111" =>
                Y7 <= I;
                Y1 <= (others => '0');
                Y2 <= (others => '0');
                Y3 <= (others => '0');
                Y4 <= (others => '0');
                Y5 <= (others => '0');
                Y6 <= (others => '0');
                Y0 <= (others => '0');
            when others =>
                Y0 <= (others => '0');
                Y1 <= (others => '0');
                Y2 <= (others => '0');
                Y3 <= (others => '0');
                Y4 <= (others => '0');
                Y5 <= (others => '0');
                Y6 <= (others => '0');
                Y7 <= (others => '0');
        end case;
    end process;
end architecture;