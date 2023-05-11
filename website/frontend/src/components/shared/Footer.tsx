import React from 'react';

type Props = {
  currentPage: string;
  setCurrentPage: any;
}

function Footer({setCurrentPage, currentPage}: Props) {
  return (
    <div>
      <nav>
        <ul>
          <li>
            <a
              href="#"
              style={{color: currentPage === 'home_page' ? 'blue' : 'black'}}
              onClick={() => setCurrentPage('home_page')}
            >
              Home
            </a>
          </li>
          <li>
            <a
              href="#"
              style={{color: currentPage === 'about_page' ? 'blue' : 'black'}}
              onClick={() => setCurrentPage('about_page')}
            >
              About
            </a>
          </li>
        </ul>
      </nav>
    </div>
  )
}

export default Footer;